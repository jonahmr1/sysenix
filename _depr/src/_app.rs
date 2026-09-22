use std::sync::Mutex;

use cocoa::appkit::*;
use cocoa::appkit::{
  NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory, NSButton, NSView,
};
use cocoa::base::{id, nil, selector};
use cocoa::foundation::*;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::*;

use crate::ai::ask;
use crate::click::at;
use crate::constants::{EDGE, FULL, HEIGHT, SHIFT, WIDTH};
use crate::locales::t;
use crate::parser::parse;
use crate::prompt::system;
use crate::screenshot::{dimensions, shot};
use crate::to_base64::to_base64;

static mut POPOVER: id = nil;
static mut STATUS_ITEM: id = nil;

static CONVERSATIONS: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());

fn inverted_y(multiplier: f64) -> f64 {
  HEIGHT - SHIFT * multiplier
}

fn objective_c_methods(decl: &mut ClassDecl) {
  extern "C" fn toggle_popover(_this: &Object, _cmd: Sel, _sender: id) {
    unsafe {
      let is_shown: bool = msg_send![POPOVER, isShown];
      if is_shown {
        let _: () = msg_send![POPOVER, close];
      } else {
        let button = STATUS_ITEM.button();
        let _: () = msg_send![POPOVER,
          showRelativeToRect: NSView::frame(button)
          ofView: button
          preferredEdge: 1u64
        ];
      }
    }
  }
  unsafe {
    decl.add_method(
      selector("toggle:"),
      toggle_popover as extern "C" fn(&Object, Sel, id),
    );
  }

  extern "C" fn quit(_this: &Object, _cmd: Sel, _sender: id) {
    std::process::exit(0);
  }
  unsafe {
    decl.add_method(selector("quit:"), quit as extern "C" fn(&Object, Sel, id));
  }

  extern "C" fn on_input(_this: &Object, _cmd: Sel, sender: id) {
    unsafe {
      let text: id = msg_send![sender, stringValue];
      let cstr: *const i8 = msg_send![text, UTF8String];
      let user_request = std::ffi::CStr::from_ptr(cstr).to_str().unwrap().to_string();

      CONVERSATIONS
        .lock()
        .unwrap()
        .push(("user".to_string(), user_request.clone()));

      std::thread::spawn(move || {
        loop {
          let bytes = shot();
          let (img_w, img_h) = dimensions(&bytes);
          println!("screenshot: {}x{}", img_w, img_h);

          let image = to_base64(&bytes);
          let prompt: String = system(&user_request, img_w, img_h);
          let response = ask(&prompt, &CONVERSATIONS, &image);

          CONVERSATIONS
            .lock()
            .unwrap()
            .push(("assistant".to_string(), response.clone()));

          if let Some(action) = parse(&response) {
            println!("clicking at: {} {} coords", action.x, action.y);
            println!("conversation: {:?}", CONVERSATIONS.lock().unwrap());
            println!("-------");
            at(action.x as f64, action.y as f64);
            if action.is_last_step {
              break;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
          } else {
            break;
          }
        }
      });
    }
  }
  unsafe {
    decl.add_method(
      selector("on_input:"),
      on_input as extern "C" fn(&Object, Sel, id),
    );
  }
}

fn sbi_handle_click(delegate: *mut Object) {
  unsafe {
    // click handler
    NSButton::setTarget_(STATUS_ITEM.button(), delegate);
    STATUS_ITEM.button().setAction_(selector("toggle:"));

    // popover
    POPOVER = msg_send![class!(NSPopover), new];
    let _: () = msg_send![POPOVER, setBehavior: 1i64];
  }
}

/* COMPONENTS */
fn sbi_set_app_icon() {
  unsafe {
    let icon_path = NSString::alloc(nil).init_str("assets/favicon-dark.png");
    let icon_image: id = msg_send![class!(NSImage), alloc];
    let icon_image: id = msg_send![icon_image, initWithContentsOfFile: icon_path];
    let _: () = msg_send![icon_image, setTemplate: cocoa::base::YES];
    let _: () = msg_send![STATUS_ITEM.button(), setImage: icon_image];
  }
}
fn sbi_popover_input(delegate: *mut Object, view: *mut Object) {
  unsafe {
    let input_frame = NSRect::new(
      NSPoint::new(SHIFT, inverted_y(5.0) - SHIFT),
      NSSize::new(FULL, 25.0),
    );
    let input: id = msg_send![class!(NSTextField), alloc];
    let input: id = msg_send![input, initWithFrame: input_frame];
    let _: () =
      msg_send![input, setPlaceholderString: NSString::alloc(nil).init_str(t("ask_sysenix"))];
    let _: () = msg_send![view, addSubview: input];
    let _: () = msg_send![input, setTarget: delegate];
    let _: () = msg_send![input, setAction: selector("on_input:")];
  }
}
fn sbi_popover_header(view: *mut Object) {
  unsafe {
    let header_frame = NSRect::new(
      NSPoint::new(SHIFT, inverted_y(1.0) - EDGE),
      NSSize::new(WIDTH, SHIFT),
    );
    let header: id = msg_send![class!(NSTextField), alloc];
    let header: id = msg_send![header, initWithFrame: header_frame];
    let _: () = msg_send![header, setStringValue: NSString::alloc(nil).init_str(t("sysenix"))];
    let _: () = msg_send![header, setBordered: cocoa::base::NO];
    let _: () = msg_send![header, setEditable: cocoa::base::NO];
    let clear: id = msg_send![class!(NSColor), clearColor];
    let _: () = msg_send![header, setBackgroundColor: clear];
    let _: () = msg_send![view, addSubview: header];
  }
}
fn sbi_separator(view: *mut Object, multiplier: f64) {
  unsafe {
    let separator_frame = NSRect::new(
      NSPoint::new(SHIFT, inverted_y(multiplier)),
      NSSize::new(FULL, SHIFT),
    );
    let separator: id = msg_send![class!(NSBox), alloc];
    let separator: id = msg_send![separator, initWithFrame: separator_frame];
    let _: () = msg_send![separator, setBoxType: 2i64];
    let _: () = msg_send![view, addSubview: separator];
  }
}
fn sbi_quit_btn(delegate: *mut Object, view: *mut Object) {
  unsafe {
    let btn_frame = NSRect::new(NSPoint::new(EDGE, SHIFT), NSSize::new(WIDTH - SHIFT, 20.0));
    let quit_btn: id = msg_send![class!(NSButton), alloc];
    let quit_btn: id = msg_send![quit_btn, initWithFrame: btn_frame];
    let _: () = msg_send![quit_btn, setTitle: NSString::alloc(nil).init_str(t("quit"))];
    let _: () = msg_send![quit_btn, setTarget: delegate];
    let _: () = msg_send![quit_btn, setAction: selector("quit:")];
    let _: () = msg_send![quit_btn, setBezelStyle: 1i64];
    let _: () = msg_send![view, addSubview: quit_btn];
  }
}

pub fn start() {
  unsafe {
    let _pool = NSAutoreleasePool::new(nil);
    let app = NSApp();

    // tells macos that the app is an accessory(no dock icon, no CMD + TAB window)
    app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);

    // status item
    let bar = NSStatusBar::systemStatusBar(nil);
    STATUS_ITEM = bar.statusItemWithLength_(-1.0);

    let superclass = Class::get("NSObject").unwrap();
    let mut decl = ClassDecl::new("Delegate", superclass).unwrap();
    objective_c_methods(&mut decl);
    let delegate_class = decl.register();
    let delegate: id = msg_send![delegate_class, new];

    sbi_handle_click(delegate);

    // sbi_popover
    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIDTH, HEIGHT));
    let view: id = msg_send![class!(NSView), alloc];
    let view: id = msg_send![view, initWithFrame: frame];

    let vc: id = msg_send![class!(NSViewController), new];
    let _: () = msg_send![vc, setView: view];
    let _: () = msg_send![POPOVER, setContentViewController: vc];
    let _: () = msg_send![POPOVER, setContentSize: NSSize::new(WIDTH, HEIGHT)];

    sbi_set_app_icon();
    sbi_popover_header(view);
    sbi_separator(view, 3.0);
    sbi_popover_input(delegate, view);
    sbi_separator(view, 8.0);
    sbi_quit_btn(delegate, view);

    app.run();
  }
}

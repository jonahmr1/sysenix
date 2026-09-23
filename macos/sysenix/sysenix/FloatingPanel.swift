//
//  FloatingPanel.swift
//  sysenix
//
//  Created by Lenix on 23/9/2026.
//


import AppKit
  import SwiftUI

  @MainActor
  class FloatingPanel: NSPanel {
			override var canBecomeKey: Bool { true }
      init() {
          super.init(
							contentRect: NSRect(x: 0, y: 0, width: 360, height: 60),
              styleMask: [.borderless, .nonactivatingPanel],
              backing: .buffered,
              defer: false
          )

          level = .floating
          isOpaque = false
          backgroundColor = .clear
          hasShadow = true
          hidesOnDeactivate = false
          contentView = NSHostingView(rootView: ContentView())
      }

      func toggle(on screen: NSScreen?) {
          if isVisible {
              orderOut(nil)
              return
          }

          guard let screen = screen ?? NSScreen.main else { return }
          let bounds = screen.visibleFrame

          setFrameOrigin(NSPoint(
              x: bounds.midX - frame.width / 2,
              y: bounds.maxY - frame.height - 12
          ))
					makeKeyAndOrderFront(nil)
      }
  }

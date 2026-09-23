//
//  AppDelegate.swift
//  sysenix
//
//  Created by Lenix on 23/9/2026.
//


import AppKit

  @MainActor
  class AppDelegate: NSObject, NSApplicationDelegate {
      private var statusItem: NSStatusItem!
      private let panel = FloatingPanel()

      func applicationDidFinishLaunching(_ notification: Notification) {
          NSApp.setActivationPolicy(.accessory)

          statusItem = NSStatusBar.system.statusItem(
              withLength: NSStatusItem.squareLength
          )
          statusItem.button?.image = NSImage(
              systemSymbolName: "sparkles",
              accessibilityDescription: "Sysenix"
          )
          statusItem.button?.target = self
          statusItem.button?.action = #selector(togglePanel)
      }

      @objc private func togglePanel() {
          panel.toggle(on: statusItem.button?.window?.screen)
      }
  }
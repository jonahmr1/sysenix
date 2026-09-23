//
//  sysenixApp.swift
//  sysenix
//
//  Created by Lenix on 23/9/2026.
//

import SwiftUI

@main
struct sysenixApp: App {
	@NSApplicationDelegateAdaptor(AppDelegate.self) var delegate
	
	var body: some Scene {
		Settings {
			EmptyView()
		}
	}
}

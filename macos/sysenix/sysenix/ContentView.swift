//
//  ContentView.swift
//  sysenix
//
//  Created by Lenix on 23/9/2026.
//

import SwiftUI

struct ContentView: View {
	@State private var input = ""
	@FocusState private var focused: Bool
	
	var body: some View {
		TextField("Ask Sysenix…", text: $input)
			.textFieldStyle(.plain)
			.font(.title3)
			.focused($focused)
			.padding(.horizontal, 20)
			.frame(width: 360, height: 60)
			.background(
				.ultraThinMaterial,
				in: RoundedRectangle(cornerRadius: 20)
			)
			.onAppear { focused = true }
	}
}

#Preview {
	ContentView()
}

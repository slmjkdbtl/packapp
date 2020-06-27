# wengwengweng

class Packapp < Formula

	desc "pack a binary to MacOS .app bundle"
	homepage "https://git.sr.ht/~slmjkdbtl/packapp"
	version "0.4.0"
	url "https://github.com/slmjkdbtl/packapp/releases/download/#{version}/packapp-x86_64-apple-darwin.zip"
	sha256 "64e5c3f5def09a03e27477a67b9305e1f8f34d6c0840e7fc91863693aa59ae74"

	def install
		bin.install "packapp"
	end

end


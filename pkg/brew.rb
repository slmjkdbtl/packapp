# wengwengweng

class Packapp < Formula

	desc "pack a binary to MacOS .app bundle"
	homepage "https://github.com/slmjkdbtl/packapp"
	version "0.3.1"
	url "https://github.com/slmjkdbtl/packapp/releases/download/#{version}/packapp-x86_64-apple-darwin.zip"
	sha256 "d254ff9a334741054147d4748d91371d0f392bcdb334c2da6dbad71f8cddf521"

	def install
		bin.install "packapp"
	end

end


# Formula for Homebrew (macOS/Linux)
# File: zenith.rb
#
# To install locally:
#   brew install ./zenith.rb
#
# To add to a tap:
#   cp zenith.rb $(brew --repository user/homebrew-tap)/Formula/zenith.rb

class Zenith < Formula
  desc "High-performance, multi-language code formatter with automatic backup and one-click recovery"
  homepage "https://github.com/user/zenith"
  url "https://github.com/user/zenith/archive/refs/tags/v1.0.1.tar.gz"
  sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"  # Placeholder - update with actual hash
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release", "--locked", "--all-features"
    bin.install "target/release/zenith"
  end

  test do
    # Create a test file
    test_file = testpath/"test.rs"
    test_file.write("// This is a test file\nfn main() {\nprintln!(\"Hello, world!\");\n}\n")
    
    # Run zenith in check mode (dry run)
    system "#{bin}/zenith", "format", test_file, "--check"
    
    # Verify the file was not modified in check mode
    assert_match "// This is a test file", test_file.read
  end
end
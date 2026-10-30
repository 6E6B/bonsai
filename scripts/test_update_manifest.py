import importlib.util
from pathlib import Path
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("update_manifest", Path(__file__).with_name("update-manifest.py"))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class ManifestTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        for name in ["Bonsai.AppImage", "Bonsai setup.exe", "Bonsai.msi", "Bonsai.app.tar.gz"]:
            (self.directory / name).write_bytes(b"package")
            (self.directory / (name + ".sig")).write_text("signature\n")

    def build(self):
        return module.build_manifest(self.directory, "v1.2.3", "6E6B/bonsai", "Release notes")

    def test_platforms_use_correct_signed_installer(self):
        manifest = self.build()
        self.assertEqual(manifest["version"], "1.2.3")
        self.assertEqual(manifest["notes"], "Release notes")
        platforms = manifest["platforms"]
        self.assertEqual(len(platforms), 7)
        self.assertEqual(platforms["darwin-aarch64"], platforms["darwin-x86_64"])
        self.assertTrue(platforms["windows-x86_64-msi"]["url"].endswith("Bonsai.msi"))
        self.assertTrue(platforms["windows-x86_64-nsis"]["url"].endswith("Bonsai%20setup.exe"))
        self.assertEqual(platforms["linux-x86_64"]["signature"], "signature")

    def test_missing_platform_prevents_publishing_manifest(self):
        (self.directory / "Bonsai.app.tar.gz").unlink()
        with self.assertRaisesRegex(ValueError, "Expected one"):
            self.build()

    def test_missing_signature_prevents_publishing_manifest(self):
        (self.directory / "Bonsai.AppImage.sig").unlink()
        with self.assertRaises(FileNotFoundError):
            self.build()

    def test_empty_signature_prevents_publishing_manifest(self):
        (self.directory / "Bonsai.msi.sig").write_text("\n")
        with self.assertRaisesRegex(ValueError, "Empty signature"):
            self.build()

    def test_duplicate_package_prevents_ambiguous_target(self):
        (self.directory / "Other.exe").write_bytes(b"other")
        with self.assertRaisesRegex(ValueError, "Expected one"):
            self.build()


if __name__ == "__main__":
    unittest.main()

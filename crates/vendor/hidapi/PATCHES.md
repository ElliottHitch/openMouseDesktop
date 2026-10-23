# Local hidapi 2.6.7 patch

Source is the published hidapi 2.6.7 crate. The original manifest, build script, source, README, license, and upstream revision metadata are retained.

`HidApi::new()` now honors `disable_device_discovery()` before its initial `add_devices(0, 0)` call. Upstream 2.6.7 unconditionally enumerates despite that setting. The Windows app explicitly indexes requested vendors instead, avoiding USB string queries for unrelated devices. Transport and descriptor implementations are unchanged.

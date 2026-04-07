pub const SPOOF_SYSTEM_PROPERTIES: &[(&str, &str)] = &[
    ("ro.product.brand", "Xiaomi"),
    ("ro.product.manufacturer", "Xiaomi"),
    ("ro.miui.ui.version.name", "V130"),
    ("ro.miui.ui.version.code", "13"),
    ("ro.miui.version.code_time", "1658851200"),
    ("ro.miui.internal.storage", "/sdcard/"),
    ("ro.miui.region", "CN"),
    ("ro.miui.cust_variant", "cn"),
    ("ro.vendor.miui.region", "CN"),
    // 建议补充型号，否则只有品牌容易被识别
    ("ro.product.model", "2201123C"), 
    ("ro.product.device", "zeus"),
];

pub const SPOOF_BUILD_PROPERTIES: &[(&str, &str)] = &[
    ("BRAND", "Xiaomi"),
    ("MANUFACTURER", "Xiaomi"),
    ("MODEL", "2201123C"),
    ("PRODUCT", "zeus"),
    ("DEVICE", "zeus"),
];

pub const CONFIG_PATH: &str = "/data/adb/mipush/app.conf";

pub const MIPUSH_PACKAGE_NAME: &str = "com.magisk317.mipush";

pub const SPOOF_MIPUSH_PROPERTIES: &[(&str, &str)] = &[("mipush.zygisk.enabled", "true")];

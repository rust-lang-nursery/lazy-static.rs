extern crate version_check;

fn main() {
    let is_var_set = |s| std::env::var_os(s).is_some();

    // one can manually set a cfg to force an impl -- mostly useful for our own testing
    let force_heap_cfg = is_var_set("CARGO_CFG_LAZY_STATIC_HEAP_IMPL");
    let force_inline_cfg = is_var_set("CARGO_CFG_LAZY_STATIC_INLINE_IMPL");
    let force_spin_cfg = is_var_set("CARGO_CFG_LAZY_STATIC_SPIN_IMPL");

    let impls_forced = [force_heap_cfg, force_inline_cfg, force_spin_cfg]
        .into_iter()
        .filter(|&&f| f)
        .count();

    assert!(
        impls_forced <= 1,
        "lazy_static can only be built with one configuration at a time."
    );

    let nightly_feature_enabled = is_var_set("CARGO_FEATURE_NIGHTLY");
    let spin_feature_enabled = is_var_set("CARGO_FEATURE_SPIN_NO_STD");

    let version_geq_122 = version_check::is_min_version("1.22.0").unwrap().0;
    let drop_in_static_supported = version_geq_122 || nightly_feature_enabled;

    // precedence:
    // 1. explicit requests via cfg or spin_no_std feature
    // 2. inline impl with newer rustc version or nightly feature (latter for backcompat)
    // 3. fallback to allocating implementation
    let impl_name = if force_heap_cfg {
        "heap"
    } else if force_inline_cfg {
        "inline"
    } else if force_spin_cfg || spin_feature_enabled {
        "spin"
    } else if drop_in_static_supported {
        "inline"
    } else {
        "heap"
    };

    println!("cargo:rustc-cfg=lazy_static_{}_impl", impl_name);
}

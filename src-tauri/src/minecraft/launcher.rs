use crate::{
    minecraft::{java::get_java, modpack::get_modpack, mojang_meta::ClassPath},
    util::DataDir,
};
use anyhow::Result;
use config::Config;
use reqwest::Client;
use std::path::PathBuf;
use tauri::api::process::Command;

use super::{
    java::Version,
    modpack::{install_modpack, ModPath},
    mojang_meta::get_minecraft,
};

pub(crate) async fn launch(config: Config) -> Result<()> {
    let data_dir = DataDir::new();
    let c = Client::builder().build()?;

    let slug = config.get_string("pack.slug").unwrap();
    let target_version = config.get_string("pack.version").unwrap();

    // Check auth from startup.rs or whatever todo
    // let auth_token = authenticate(something) (auth.rs)

    // Get modpack FIRST.
    // then get java & minecraft with specified version in mrpack

    // Get java from config/download it
    let java = get_java(Version::from_mc_version(&target_version), &data_dir, &c).await?; //done (ish)

    // Download minecraft version. Install, libraries, assets
    let class_path = get_minecraft(&target_version, &data_dir, &c).await?; // done

    // Get pack from config
    get_modpack(&slug, &target_version, &data_dir, &c).await?; // done

    // Download quilt/fabric/legacy fabric/whatever
    // Make .minecraft at [DATA_DIR]/instances/MCVER/.minecraft/
    // Download mods from mrpack into .minecraft
    let mod_path = install_modpack(&slug, &target_version, &data_dir, &c, &class_path).await?; // no

    launch_minecraft(
        &java,
        &data_dir,
        "net.minecraft.client.main.Main",
        &slug,
        &target_version,
        &class_path,
        &mod_path,
    )
    .await?; // no

    Ok(())
}

async fn launch_minecraft(
    java: &PathBuf,
    instance: &DataDir,
    main_class: &str,
    slug: &str,
    version: &str,
    class_path: &ClassPath,
    mod_path: &ModPath,
) -> Result<()> {
    // java -Dfabric.addMods=:::::::$HOME/.axolotlclient/common/modstore/Axolotlclient-1.8/required/AxolotlClient-2.1.6%2B1.8.9.jar:$HOME/.axolotlclient/common/modstore/Axolotlclient-1.8/optional/old-block-hit-0.2.2.jar:$HOME/.axolotlclient/common/modstore/Axolotlclient-1.8/required/
    // -cp $HOME/.axolotlclient/common/versions/1.8.9/1.8.9.jar:$HOME/.axolotlclient/common/libraries/net/legacyfabric/intermediary/1.8.9/intermediary-1.8.9.jar:$HOME/.axolotlclient/common/libraries/net/fabricmc/yarn/1.8.9+build.202201302314/yarn-1.8.9+build.202201302314.jar:$HOME/.axolotlclient/common/libraries/net/fabricmc/tiny-mappings-parser/0.3.0+build.17/tiny-mappings-parser-0.3.0+build.17.jar:$HOME/.axolotlclient/common/libraries/net/fabricmc/sponge-mixin/0.11.2+mixin.0.8.5/sponge-mixin-0.11.2+mixin.0.8.5.jar:$HOME/.axolotlclient/common/libraries/net/fabricmc/tiny-remapper/0.8.1/tiny-remapper-0.8.1.jar:$HOME/.axolotlclient/common/libraries/net/fabricmc/access-widener/2.1.0/access-widener-2.1.0.jar:$HOME/.axolotlclient/common/libraries/net/fabricmc/fabric-loader-sat4j/2.3.5.4/fabric-loader-sat4j-2.3.5.4.jar:$HOME/.axolotlclient/common/libraries/com/google/jimfs/jimfs/1.2-fabric/jimfs-1.2-fabric.jar:$HOME/.axolotlclient/common/libraries/org/ow2/asm/asm/9.2/asm-9.2.jar:$HOME/.axolotlclient/common/libraries/org/ow2/asm/asm-analysis/9.2/asm-analysis-9.2.jar:$HOME/.axolotlclient/common/libraries/org/ow2/asm/asm-commons/9.2/asm-commons-9.2.jar:$HOME/.axolotlclient/common/libraries/org/ow2/asm/asm-tree/9.2/asm-tree-9.2.jar:$HOME/.axolotlclient/common/libraries/org/ow2/asm/asm-util/9.2/asm-util-9.2.jar:$HOME/.axolotlclient/common/libraries/com/mojang/netty/1.8.8/netty-1.8.8.jar:$HOME/.axolotlclient/common/libraries/oshi-project/oshi-core/1.1/oshi-core-1.1.jar:$HOME/.axolotlclient/common/libraries/net/java/dev/jna/jna/3.4.0/jna-3.4.0.jar:$HOME/.axolotlclient/common/libraries/net/java/dev/jna/platform/3.4.0/platform-3.4.0.jar:$HOME/.axolotlclient/common/libraries/com/ibm/icu/icu4j-core-mojang/51.2/icu4j-core-mojang-51.2.jar:$HOME/.axolotlclient/common/libraries/net/sf/jopt-simple/jopt-simple/4.6/jopt-simple-4.6.jar:$HOME/.axolotlclient/common/libraries/com/paulscode/codecjorbis/20101023/codecjorbis-20101023.jar:$HOME/.axolotlclient/common/libraries/com/paulscode/codecwav/20101023/codecwav-20101023.jar:$HOME/.axolotlclient/common/libraries/com/paulscode/libraryjavasound/20101123/libraryjavasound-20101123.jar:$HOME/.axolotlclient/common/libraries/com/paulscode/librarylwjglopenal/20100824/librarylwjglopenal-20100824.jar:$HOME/.axolotlclient/common/libraries/com/paulscode/soundsystem/20120107/soundsystem-20120107.jar:$HOME/.axolotlclient/common/libraries/io/netty/netty-all/4.0.23.Final/netty-all-4.0.23.Final.jar:$HOME/.axolotlclient/common/libraries/com/google/guava/guava/17.0/guava-17.0.jar:$HOME/.axolotlclient/common/libraries/org/apache/commons/commons-lang3/3.3.2/commons-lang3-3.3.2.jar:$HOME/.axolotlclient/common/libraries/commons-io/commons-io/2.4/commons-io-2.4.jar:$HOME/.axolotlclient/common/libraries/commons-codec/commons-codec/1.9/commons-codec-1.9.jar:$HOME/.axolotlclient/common/libraries/net/java/jinput/jinput/2.0.5/jinput-2.0.5.jar:$HOME/.axolotlclient/common/libraries/net/java/jutils/jutils/1.0.0/jutils-1.0.0.jar:$HOME/.axolotlclient/common/libraries/com/google/code/gson/gson/2.2.4/gson-2.2.4.jar:$HOME/.axolotlclient/common/libraries/com/mojang/authlib/1.5.21/authlib-1.5.21.jar:$HOME/.axolotlclient/common/libraries/com/mojang/realms/1.7.59/realms-1.7.59.jar:$HOME/.axolotlclient/common/libraries/org/apache/commons/commons-compress/1.8.1/commons-compress-1.8.1.jar:$HOME/.axolotlclient/common/libraries/org/apache/httpcomponents/httpclient/4.3.3/httpclient-4.3.3.jar:$HOME/.axolotlclient/common/libraries/commons-logging/commons-logging/1.1.3/commons-logging-1.1.3.jar:$HOME/.axolotlclient/common/libraries/org/apache/httpcomponents/httpcore/4.3.2/httpcore-4.3.2.jar:$HOME/.axolotlclient/common/libraries/org/apache/logging/log4j/log4j-api/2.0-beta9/log4j-api-2.0-beta9.jar:$HOME/.axolotlclient/common/libraries/org/apache/logging/log4j/log4j-core/2.0-beta9/log4j-core-2.0-beta9.jar:$HOME/.axolotlclient/common/libraries/org/lwjgl/lwjgl/lwjgl/2.9.4-nightly-20150209/lwjgl-2.9.4-nightly-20150209.jar:$HOME/.axolotlclient/common/libraries/org/lwjgl/lwjgl/lwjgl_util/2.9.4-nightly-20150209/lwjgl_util-2.9.4-nightly-20150209.jar:$HOME/.axolotlclient/common/libraries/tv/twitch/twitch/6.5/twitch-6.5.jar:$HOME/.axolotlclient/common/libraries/net/fabricmc/fabric-loader/0.14.4/fabric-loader-0.14.4.jar
    // -Xmx2500M -Xms2500M -Xmn128 -XX:+UseAdaptiveSizePolicy
    // -Djava.library.path=/tmp/AxolotlClient-temp/264c74ec822d566bfc63a99655e49e96
    // net.fabricmc.loader.launch.knot.KnotClient
    // --username <username> --version Axolotlclient-1.8 --versionType AxolotlClient
    // --gameDir $HOME/.axolotlclient/instances/Axolotlclient-1.8 --assetsDir $HOME/.axolotlclient/common/assets
    // --assetIndex 1.8 --accessToken <accessToken> --uuid <uuid> --width 800 --height 480

    // todo: add mods, add fabric libs to classpath (including fabric loader!)
    Command::new(java.display().to_string())
        .args([
            &format!(
                "-Djava.library.path={}libraries",
                instance.path.canonicalize()?.display().to_string()
            ),
            "-cp",
            &class_path,
            main_class,
            "-Xmx2048M",
            "-Xms2048M",
            "-Xmn128",
            "--gameDir",
            &instance
                .get_instance_minecraft_dir(slug, version)
                .canonicalize()?
                .display()
                .to_string(),
            "--assetIndex",
            version,
            "--assetsDir",
            &instance
                .get_assets_dir()
                .canonicalize()?
                .display()
                .to_string(),
            "--accessToken",
            "0",
            "--uuid",
            "0",
            "--version",
            version,
        ])
        .spawn()?;
    println!("Game launched.");

    Ok(())
}

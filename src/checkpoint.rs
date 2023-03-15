use wasm_bindgen::prelude::*;

//Return the closest checkpoint
pub fn get_checkpoint(block_height: i32, is_testnet: bool) -> (i32, &'static str) {
    let test_network_checkpoints = [(0,"000000"),
(43200,"0145b24b3fb3fb5fee896cf286f121cc53b9b2b157769623ae94bf67a0e52595320006017c71fcc62766c3d9a714a42e7a7e1f03cd0da795cb88f9682596bc9e4df0794b015417eb43736a3766008063dddfa3aaeb41c0f3462d897cff901b781de54a905601497aa2027dbbddccd3791b6461167a80a6eb3892c004890dc9cdbab326a9fa3d000001a386834c8c3053059f4c4f1737bd2c35ac2b603d2014885057d56d634782f50f"),
(86400,"0162efe4828d24c3a8ed9e06a4746f62d8f09a48e4bfa2d30efd2fd7e4b6464713000600018ca3be8edd0396d9d90fe85cfca8ccdd76ade4e05119c8a9135453dfdd62a57301459cd8b7aeb738d35c4578e3f6044a539cf3c7467fd64dd0d15ba32f87d2325e0195d553b95d9a9c1f074e92c144ebde9d795b0b2cf01304e7d9010704346f664f0001a386834c8c3053059f4c4f1737bd2c35ac2b603d2014885057d56d634782f50f"),
(129600,"01e66654c3b071ef12ee59595e8153bd0af293760e68b02fad35e9de29941245280152d73cbf520a19f75e4fd83f49e33d51aa6d20d51565a8f42e213b39a61690630700000155877797b744c606247569dd75db8291a4e1a12a55306800c390198f41acf36f0106760ca056835b8ad9562dd86c5a63752a7eda3fc2b59395965bd96b2d42c52000000108fcb39bc7a296b148f21e2392a124b7760578d6abfd0ed73ebe2cc8dbca6c32"),
(172800,"01e74455d0f2f1967a796b623d632136193b79f775dca74d9ebd3e708a15a53a0f017d0bce93a5115fd6da8af65b590d6bcb789decb2ec1d84e83aecf22e1cce754b070000000001b1cbb323d3591f4992a42b0eb37a30f6b7a91fe02919730d910f9210e917f321000108fcb39bc7a296b148f21e2392a124b7760578d6abfd0ed73ebe2cc8dbca6c32"),
(216000,"01162662f86a6b5ad4548e454ee768c66817f971699b04d6bcbed3848c4ef5da6501de5e619f5b5ee88c2f00c6b774819bcaedab9baeb83aa54b1f7d69d8f48eca5c0701d5dfea136e30dde4b8e0ecd1b9e7df562b7f6c1d61a3b667a90cb3ec4103f6570001b228b06a7124e056bd50a8078c3a0952f98e3f7518405b135c87513a80fd710b0001b1cbb323d3591f4992a42b0eb37a30f6b7a91fe02919730d910f9210e917f321000108fcb39bc7a296b148f21e2392a124b7760578d6abfd0ed73ebe2cc8dbca6c32"),
(259200,"0133a9062212bda2861bfc67e705b3d11fc9483457d5771c63180efed3f214a841015f7c601a783a775796b66734b785a8574ff56781ff49af8adb2863f50971431707000001b7d0781dd5bfecb7b1ef9031b435cb9c7c316c4f6919541d731bddbae9dad84d01f09b68ed8eb945bb33746ce8fa642b4e48acaff950c0b3a24d1f175c8f70a64b01b1cbb323d3591f4992a42b0eb37a30f6b7a91fe02919730d910f9210e917f321000108fcb39bc7a296b148f21e2392a124b7760578d6abfd0ed73ebe2cc8dbca6c32"),
(302400,"0107cd822c871bbb8c9b871d3910011e4e2462861ef30ff25addfde888bd887d4d019bda186bcc085145173d6090dfdc9540e1ecfb268e9e7c088874e04e1b3ec71c0d0001cc7de2a823ad423da696e6438aee581245e34da5478e1329230bb168bb35a00b00000001707f24b1350fa2e6a58fcdd0af027a9f6ccf578b8e02172f56f1d1957773ef40000000010570f945f322cb538d140b50ede177ad6693924e6f446b20dd630814dcee8319000001e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(345600,"01041f374ac805c9f91595e1ddb3ac092a7f89c81692ac3e13b9117eecc8c6863b01d291f6499a5e5df3263ebdcf91f864359b6a80b0304b8665d97e726b1a6565570d0000013545bfe735a6b892a3399104d13d00afb86b6e87475f3c9de6b24550aa431e4b000001707f24b1350fa2e6a58fcdd0af027a9f6ccf578b8e02172f56f1d1957773ef40000000010570f945f322cb538d140b50ede177ad6693924e6f446b20dd630814dcee8319000001e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(388800,"010855a87e5be9e989097c8671d3229db31c150d4937a5d013b16b5ae8dcb8b836013200eaefae40491f7283c9e932c1f3c00ee8811786ef7c7e6b86347717127c5a0d012d7d8c1eca82da7320e48e645a6babc5da01f33957ac30dcee1093589961fa17012e076f80fbf36ff1f6473945dbbdb4157f9591f061a451f184277e6811e53b27013545bfe735a6b892a3399104d13d00afb86b6e87475f3c9de6b24550aa431e4b000001707f24b1350fa2e6a58fcdd0af027a9f6ccf578b8e02172f56f1d1957773ef40000000010570f945f322cb538d140b50ede177ad6693924e6f446b20dd630814dcee8319000001e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(432000,"01ff092a238b21d93eb098ab76e362b54b22c76a1e6d5c0a595a63265ddc7c0057000d01cb7e833b0cc95dc362ce68b3b0068ba1c7196373e630d1274089d6421f2276340001850e67cb6e6907841095c26fd71d2731a843e57a60c66ee39c99be3eaef7284201c0eefbfc823e0725ccc2d0cfd42cdc1083643a70faed0eb77d2d4fb45467be15000177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(475200,"0188406b1247744c09648f28cedc7f39beb9f19b0f0addbebed5c4b3c7b26ea908015e4e06906da3db91e661b58e6a19e0b18185be808d393e19711c4d6ef3cb0d460d015f7b07f3e6ba1d1ccf6c67894e7b75184213815e35efb8c271d833b09d1c86290001ac87dcd094df51400552f888780782430f9bb37d717c42c358703f3aa5bc69190001494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(518400,"016998bb16fe41ec76fd6f6595a798d2b2dc94c76e1bdf00c3b2246b3e9969292001277bd4b22eb0cc1ab756622685f3565985748c8c753a3b5deb47a53f724f19730d0001e9381c29f492821b224dff42a3cd0d8685dc465d0247195cd78a8c1a372cba2a01ac87dcd094df51400552f888780782430f9bb37d717c42c358703f3aa5bc69190001494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(561600,"0155fd9f7a045b1d2e4a5ef9d15dc0af0e3427a6bc96741783366cbee081848659000d0188753be103e487a22575a042bf5155cd58d5db83e4bb3fb12ec7936ff31fcc4201e9381c29f492821b224dff42a3cd0d8685dc465d0247195cd78a8c1a372cba2a01ac87dcd094df51400552f888780782430f9bb37d717c42c358703f3aa5bc69190001494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(604800,"0155fd9f7a045b1d2e4a5ef9d15dc0af0e3427a6bc96741783366cbee081848659000d0188753be103e487a22575a042bf5155cd58d5db83e4bb3fb12ec7936ff31fcc4201e9381c29f492821b224dff42a3cd0d8685dc465d0247195cd78a8c1a372cba2a01ac87dcd094df51400552f888780782430f9bb37d717c42c358703f3aa5bc69190001494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(648000,"0155fd9f7a045b1d2e4a5ef9d15dc0af0e3427a6bc96741783366cbee081848659000d0188753be103e487a22575a042bf5155cd58d5db83e4bb3fb12ec7936ff31fcc4201e9381c29f492821b224dff42a3cd0d8685dc465d0247195cd78a8c1a372cba2a01ac87dcd094df51400552f888780782430f9bb37d717c42c358703f3aa5bc69190001494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(691200,"0155fd9f7a045b1d2e4a5ef9d15dc0af0e3427a6bc96741783366cbee081848659000d0188753be103e487a22575a042bf5155cd58d5db83e4bb3fb12ec7936ff31fcc4201e9381c29f492821b224dff42a3cd0d8685dc465d0247195cd78a8c1a372cba2a01ac87dcd094df51400552f888780782430f9bb37d717c42c358703f3aa5bc69190001494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(734400,"0155fd9f7a045b1d2e4a5ef9d15dc0af0e3427a6bc96741783366cbee081848659000d0188753be103e487a22575a042bf5155cd58d5db83e4bb3fb12ec7936ff31fcc4201e9381c29f492821b224dff42a3cd0d8685dc465d0247195cd78a8c1a372cba2a01ac87dcd094df51400552f888780782430f9bb37d717c42c358703f3aa5bc69190001494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(777600,"0155fd9f7a045b1d2e4a5ef9d15dc0af0e3427a6bc96741783366cbee081848659000d0188753be103e487a22575a042bf5155cd58d5db83e4bb3fb12ec7936ff31fcc4201e9381c29f492821b224dff42a3cd0d8685dc465d0247195cd78a8c1a372cba2a01ac87dcd094df51400552f888780782430f9bb37d717c42c358703f3aa5bc69190001494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(820800,"01a7f16918d5508076edd7acac3726c2b46da807d85bcb6e3a19501ebabc75f040017d457c488fef8e60c3b7bad797b679041b118e7dbaf2a0dd5e93e45f87d39a220d00018407f6460507332c0e19d7d8333873e8038b85d82425e201b13b29a3f75974120001a3ce8339e68a22d91b0750ef45468efe763e3d5e3a6e59809ddadcc94fe73c6c01494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(864000,"01a7f16918d5508076edd7acac3726c2b46da807d85bcb6e3a19501ebabc75f040017d457c488fef8e60c3b7bad797b679041b118e7dbaf2a0dd5e93e45f87d39a220d00018407f6460507332c0e19d7d8333873e8038b85d82425e201b13b29a3f75974120001a3ce8339e68a22d91b0750ef45468efe763e3d5e3a6e59809ddadcc94fe73c6c01494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(907200,"01a7f16918d5508076edd7acac3726c2b46da807d85bcb6e3a19501ebabc75f040017d457c488fef8e60c3b7bad797b679041b118e7dbaf2a0dd5e93e45f87d39a220d00018407f6460507332c0e19d7d8333873e8038b85d82425e201b13b29a3f75974120001a3ce8339e68a22d91b0750ef45468efe763e3d5e3a6e59809ddadcc94fe73c6c01494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(950400,"01a7f16918d5508076edd7acac3726c2b46da807d85bcb6e3a19501ebabc75f040017d457c488fef8e60c3b7bad797b679041b118e7dbaf2a0dd5e93e45f87d39a220d00018407f6460507332c0e19d7d8333873e8038b85d82425e201b13b29a3f75974120001a3ce8339e68a22d91b0750ef45468efe763e3d5e3a6e59809ddadcc94fe73c6c01494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(993600,"0192f0897166869566a11e267c3430a2714df579d94eaa9fd4c8a5174073d87b1b000d014f4c17968939cde8739bd08f7486618e4e440cba61653bf84434ac356b77d36b0001e105bf42db29eca36e7235bd55546726753d1f967c3f284e243cbb3b3375d95a01a3ce8339e68a22d91b0750ef45468efe763e3d5e3a6e59809ddadcc94fe73c6c01494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(1036800,"0192f0897166869566a11e267c3430a2714df579d94eaa9fd4c8a5174073d87b1b000d014f4c17968939cde8739bd08f7486618e4e440cba61653bf84434ac356b77d36b0001e105bf42db29eca36e7235bd55546726753d1f967c3f284e243cbb3b3375d95a01a3ce8339e68a22d91b0750ef45468efe763e3d5e3a6e59809ddadcc94fe73c6c01494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(1080000,"0192f0897166869566a11e267c3430a2714df579d94eaa9fd4c8a5174073d87b1b000d014f4c17968939cde8739bd08f7486618e4e440cba61653bf84434ac356b77d36b0001e105bf42db29eca36e7235bd55546726753d1f967c3f284e243cbb3b3375d95a01a3ce8339e68a22d91b0750ef45468efe763e3d5e3a6e59809ddadcc94fe73c6c01494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),
(1123200,"018c325f63f5cc98541cfef957f64845c86cf928e317ecc71a14debd364c7b8f57013c6f50deb5f788d5ac9105915ab9cbcda21a101d267c6424aa75b6e8df969e480d00016a2b0e3728a820b7982d81c87b80468ce65a4081843b890307115ca896416f3901e105bf42db29eca36e7235bd55546726753d1f967c3f284e243cbb3b3375d95a01a3ce8339e68a22d91b0750ef45468efe763e3d5e3a6e59809ddadcc94fe73c6c01494803bd8e6b730cb277701c613a4e7355cb54f79653724618e1436c02fca30c0177d25b5ed812af45eb46b54bc37c3fbe08fdfb4d952bb917fe59187bc78c42640001088b8a9fc4769017f3fdf865637e5cebbeaf7a4c643247723bf009da5eb1e4340001e877753448933a336fcf9399cc3dcd357344510c79db717e976979cb2eab612d0001cb846820acd916b4ea03b0a222b3eae8704bbd5365f105156041c578bd214c3201e03719b3810c7a9eaf6680ad3c60fb5ffdb0106975c952ef173c3e8cde943b03"),];
    let main_network_checkpoints = [(2700000, "000000"),(2721600,"01c09aa4e61573225ea0f53d3e3a42d936b01bd3e356e306af4ca7a5151e3f10120008012b8e4658cb43820be7aa77711aca3a757cb145c0c02dcad6338717c8d8505f20000000013c87edc1e3128abdf8572d0e3ac660e79109cee4259a0bd9c3a00cea877e675e01b1853a4ee1c97d9e7f8ae27085273dfbabac9d68cd709c65404b5d989c016153016809046c4254c12dad316b4e23f52e1ce80a9a53e99261f179eb02f065368f150147145eab86095e8844966cc1d4ef54a23692ea5b19d19cd41cdf0c83006ee24e"),
    (2764800,"01a015b6a4d834024a11d014fd97161231661fd21888b665e2fa827627fb753915000a000001ca98732ec24fd29cab1796f4e7c45272e744d996c417dae3aad4a032ebe33b44000179f083ecc08e4e465e955481c176ec7ef1d9e04f915e5ac18330e41a9207684f01f244112b35f6007ed6154406c251efa73c57dd4acf1c96a9a92990354edca86d019e594e0159774eae1727cfcf811a9dadb9c3b758af08bd47a3977c4f4476093b00000151fddddbe55ded65c7fb68bc127b0d2c254838e7ffab6c15bca6454b25faa84f"),
    (2808000,"0178887510ab1577d380f386c9692ce2c3525fcd154436436adad198e86fe2f16901b69ced08f87e751bd0064a99c588950c909b4c8515d4ac549172ba6bca52c45d0a0001567b59cc703fc000bd51f9ae31da426a3a3e1d900a3826ff2fab285584504a4e000001de4771af6f869143c7d901f9d2b5b645a61e0d699fdf850ffb4d3eac6af4263d00010102501ab928d82f8f278d16f978fd0a840a1530b8faf9e04ea6b35d25f564210001e6f76c84e55f838f476bcd9d75f40595f4a5eb8999d83ebfe7a5b902729732710151fddddbe55ded65c7fb68bc127b0d2c254838e7ffab6c15bca6454b25faa84f"),
    (2851200,"017435817a838fc49d1163ab2b5949ecd7fc94743596f10970dbfc3af47422c64801ae4cde6f0d51b505162bcc7a49be8f2686f4a4c4d7429a467cb8207cb4c2f1620a01beb78b25170025a410cd21a6d91b042e26bb11f06a4164acb5229d718c9d00590001886c1df35906aaba27a64156166e6ea1bc1b6f29e7af0e4208563f4da341af35000001e3828b58238442968630bada80f1fb7c8c8e84b1705f33fb654061eece2e2e4c011caaf6b7a135ced3c9a7d680aa0c9d19975309c4182f3c5e112a3f5200e0fb590137248f1653b7061cffbb0b19e6092a059e9dba66790dee4bb18433ccd142d31501e6f76c84e55f838f476bcd9d75f40595f4a5eb8999d83ebfe7a5b902729732710151fddddbe55ded65c7fb68bc127b0d2c254838e7ffab6c15bca6454b25faa84f"),
    (2894400,"0164f0e0815d6e9052c28351429985813309367ea8345a806f29761d25a343ef050170f800768460b1b08ef9ecccf11795a6c08803dad7d9c336e6ee262f53ffed040b0190737d5899594576f732035c08250448302ecb73daee9e54346f20e8d7fdd81f00011452730c32c63fd65821e8a206287e151d6edf03daefdb30b0698a469a4e9572000001db87d5338d48586251becefd0d7f3796cae3458edd4dbbdd27bceaecd8fcd56800010764b06929330922f1e1a64b7d88cfc9ff078f03efca5ac3fb4823a7f9a2672f000001bef211c0fd1e3a8803b69652d2d7e54e32ab3802da11a45ee0761afe07a0a44d"),
    (2937600,"0199540aae4ff663b2dc9fdbd08ff0f873663233dcf9928e8377f577f2207c3468018bb32d1963a25f604f237dcdc5fd05274682744aabf0b667cd9d4a1569f85b410b0000000154f3518b003c973b4d165b36ee477881f780f5f47750ad7740ed1bfd4cce426e00013ccfbb9624cb5a6f970d922a41a0b86d7fda7692ae655d5872ec421e6922f93d01ca5ea010b6b34b515c16d0df8ba5656132f218a12867d11c7f038ccf747432650001dda36017bbca5d7864c0c0d8735d63e7e08fb962369df71e0d6dd390809ce0610001bef211c0fd1e3a8803b69652d2d7e54e32ab3802da11a45ee0761afe07a0a44d"),
    (2980800,"01ca5e23d28f49e01dce85b5080c578411d32ffcb94462fb8951ef49b51f690b5c000b0000000000013888e7bb5a6f7c60dfa81fd6548ef0f5f0ca72d6020b6003f5244895566fb6360000000136e540644c0089b61eba6f425ef76aff0cf6c633938608909d24d0fdc002426d01bef211c0fd1e3a8803b69652d2d7e54e32ab3802da11a45ee0761afe07a0a44d"),
    (3024000,"0192c84ee49fa02c49646761195c0a31bbe153529d700f7b14a49f274ff3614c5b000b0175fe378a94699c0341ceca762e660f744da742d905cb9d05323974612a5ca953016dd04716d1ae6b3daa78d39a2ed83456b7ed9890bc817a8cc238fe017fb9bf6e0001fddd03403139b686f60da9339bb7f943b5711abd56896004e9d952139ae233310001207f861b0b416554cbafb09cdf2abe06a5d5bf60a7f6a3ec3deddbaa5a79962b018bda6ad570ccf6b895070a94d542bc4746410effa2a9fc89d4e0ce804437756c013df74bd573e8066dcef9b63487c956adfcbe50c88d5d40c54434b19025b9d45f000136e540644c0089b61eba6f425ef76aff0cf6c633938608909d24d0fdc002426d01bef211c0fd1e3a8803b69652d2d7e54e32ab3802da11a45ee0761afe07a0a44d"),
    (3067200,"01a060924bf53ad9e1aeb5ca38980551013cc335089e7242b4b85d5a85f16260400161ad793da5e431743137249390d845ca243ee76ccc58f6d66437a863e4a5cc590b0124c96e7f4063bb7e543e98efec25cf1d73b5f9faee59fed6dc6c9da8f9158c4100000170a93849717b8d7366c786523c97a5c0933ddb979e2ccbf50a231217a5bcc06001163d9b901aa31b500113c8e31fe5e2e183401bd821067322f8042682f21f401d0000016a1ab3751b4d404376d333f7780c89ffd74160e63b99e3bb73c59a556f2178090177ed839d7ca27723bbca2cf177337315acc86119e5e842bfae824c5688d52d560136e540644c0089b61eba6f425ef76aff0cf6c633938608909d24d0fdc002426d01bef211c0fd1e3a8803b69652d2d7e54e32ab3802da11a45ee0761afe07a0a44d"),
    (3110400,"015968836a5886a87a4c7f3acf1b7339c46bbe25e9cab7d89510f6ba1a8bdbb30d01b89e3aa0eda9a841a7633f653e84280746bc72044abdf6bd3d2db0db84f6a73e0c0000000001e5dbaa804ddfb0094f6afc9ed362112b39bec4b149f65fb81f245a53c9d7844600000000000001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3153600,"0125924580de7472249937ccc56d986fd147863383b3cac56e9280c6859d05490801cc41a4a670588b12212b856bc1a494ab9322ef073fe5a52c1942aebd0903df0d0c00000130b89e8eb6f152c5b1f15b19c0e6ef0f9727ddc4307e1a2ee87daf6168f5b06601f21de2b11d0cd078edb3191a29a6e108afcf742d015755dd955f5f323892ba1901334f07a5b0bcd03920d48c4f8fefca10c672568f5b44fc6a5d4c1f201143c222000196c216d9d6bf190c20dc56601ff99767b8a84f79ff5c5f41d92e352d958f1211012aebb60f06b2853464e27813ccc27c95fd464f7f5089d880ced4802be4296e3000000001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3196800,"011a00fd7a6c2be1b3194db1b9e16ef035bd5d635394f36f0a97f86272d65e16200175ca47a36dbb0ee6fdd84d9955f50954dfc56b1d5c0df82ab418fef422253b1a0c01cf75bb2756178e900e9893c61d82191a2f87ce336ccfbec6d0c06ba88f0a8e53018e5828866bd7c92480fe628139b911617cc46f34330e88421ae968dec8a90b070000000146fe8a64bda3c3da281444464a98ba04f5b6638f5d58fdb71dff0f3186df591201181376c9a8c75ba680e13d8c9e72e28585573bbaf6ccd19bee946098d36ad2040001f376f9de981a8b4037825f0974248681a995d3a0de2d970170c6fc7b23e09203000001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3240000,"0132915501f7426a57f74a250f84e621093ffc74ca2d6560ba9aa3b3b42214cb0a000c00000001abcd12613f9b6ad53d22c45897581b145a12a0fc027aebec33653f7ed866832100000000000125813f3ac86de24525d7a5d69920d993eb4cb63da0ebd84171ae044fb4f39a490001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3283200,"01956e48553321d206db0e489cf92a2bddbc911c7e716d3b01e596c51c7421630d000c0160f9b233594b15c99e3cd505ceebe74dcb6c88e127ddf8865361783317608c5d0001d8ce29d1bc8b81f30996fb91f71430cb10ed7a38fe5759d673565e1ef67cd24c0001c441ab432841543bc3a2cbeef3d6b736baf55e973cd656f145adf670471ecb5d000001d66336ebc0cbbba189e75562773576e720a7921c055075cb56170cb36126fd1c000125813f3ac86de24525d7a5d69920d993eb4cb63da0ebd84171ae044fb4f39a490001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3326400,"01f7239cd6e982bd3bc945dc6988ae76ed708b029018c45c7c21ab5dbc7020e25f000c000001996a704d4f731981da758801be1a0266e2bd6ee50b4b920f5a94254aa6b19c3a0001e8786381b4ec91a3ec9a81ddb26312dde410781d3eaff6af2bc04fa8910c010b0179c89f01c246ed879dffd06a01d20f2dfb0d3e13467dffa41c773ed79ec2390e01ad363c43174b59ae1919f0e998e9f59400b7ff935762b81504e6b9283fd0695d01d66336ebc0cbbba189e75562773576e720a7921c055075cb56170cb36126fd1c000125813f3ac86de24525d7a5d69920d993eb4cb63da0ebd84171ae044fb4f39a490001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3369600,"01935032fb261c2112bc7119c7d12dd0c2793054235a9c81f0a0cf4d185a899760000c018cd2f357a6900454c3a0db7c21581c0a76a410d1100c994a244a5f42c7b53f400121ea6b375432b16f05191e73144df29ca339050b69adf808645eefdda5901d5401b14ee7c45652581128e42f72671f0372536c974888fee85d7551d3512c39b51201a7910f79aa7252fd526843877c19849df49e5fcbb946ef30b58c46d3254eaa21015793d93f90bdba9802d35ca39837a65eac6cc9c60a3209f0f0443471ca6e122000000001a7617d577a4dd87340361607ce728dd480af9c70097fb05d4ace4a7b6de393450125813f3ac86de24525d7a5d69920d993eb4cb63da0ebd84171ae044fb4f39a490001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3412800,"0135fc21aa630a53b6ded86a8521efff0f8646e25f858df1e0591120215642d525014dc6d40aa767d667bf5899c250661ee33bdcf78cc57a8d3ea5963400961079630c01816d2c50f5e3716b238ca7a300d9349bc333dd3cd244cb6c717807a571e6c729017f60ee06acbcc04eca2c8e5341d19f773655f7cd5523fff7547567ef49ce425f01860b7e64113c65a03e55e0e7c09daf75133a8af470dd323e6c0e24e8477ad534000000012284c9d3a177a3e0ec3ea137678a73bf2ef5d7c6ef2505c09168acb8700e9d150001a7617d577a4dd87340361607ce728dd480af9c70097fb05d4ace4a7b6de393450125813f3ac86de24525d7a5d69920d993eb4cb63da0ebd84171ae044fb4f39a490001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3456000,"01e819b38b8b2f9044658bd5f1b3dc0afed50c9c1ff3c6f2bf4cf36de33c8a8025013cc97c746cbd72dc8fa589cf615107525599a0fbac02cc915a56d786c9d604670c00013e82c72cfb6f2d23812d71b9208f0d37da75febe536cb37cdbe72dd2d22c08520165179a57a92c3fef8bfcbc1944b57dc9ac230c896c631a8e22b5c2f80ea44f63017b3d5630c448c613aa9aeae776deed1d5c908f37051ec00c59e5325fb64d555401087b39401002ad296991bc8606407c6bffc1a922f4a84c84ed03756a7dbcbc3801a38e5a110694e6adf7ec5c4dda533c869cf81c48c45be2a7740c4c552825912e012284c9d3a177a3e0ec3ea137678a73bf2ef5d7c6ef2505c09168acb8700e9d150001a7617d577a4dd87340361607ce728dd480af9c70097fb05d4ace4a7b6de393450125813f3ac86de24525d7a5d69920d993eb4cb63da0ebd84171ae044fb4f39a490001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3499200,"017a1ac3f48116f7fe4492095406fbe0cdb1345de095c9607c2f0460764fe13e400174dd4d9eed48bfbb42985cdcfb7f838e1c3d9d79c686fff2f35d98d7a495f82d0c00000001f4cc144370d9b74d2e3e687cabdc2c4de36e1e3372accf6ee6b1742ca1d3a94300017dd91792866a6fb7746c3331bd9fab11dee61ed1fc7d631a0a2d7a87dd1a3427000170b1352b6f8752f930501e2371d380a8bbeb5447bd9e9828d2a877ead9ad6d6801a7617d577a4dd87340361607ce728dd480af9c70097fb05d4ace4a7b6de393450125813f3ac86de24525d7a5d69920d993eb4cb63da0ebd84171ae044fb4f39a490001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3542400,"01e8129be2170c40fff557c93cfbdeb94dd6ded617f888d812577414d033b9ab0d000c017c78e027699e0b4b813139b1bf51b868c5565bab73070d9b706ade8c65fcf047017911bb4f9a739e61210609e0a2da6983de85b9ffd1f05cfbc05f52528ecb4420000000015770ffd0ffa0743590816a5b53bcc36f9f4bfe582756d7c08fda22fd5b57c423014a1f4cbe6a4e2774dc8079c9eb69af80ef2a7c4a0c48b781efa0bef9692c891b0170b1352b6f8752f930501e2371d380a8bbeb5447bd9e9828d2a877ead9ad6d6801a7617d577a4dd87340361607ce728dd480af9c70097fb05d4ace4a7b6de393450125813f3ac86de24525d7a5d69920d993eb4cb63da0ebd84171ae044fb4f39a490001cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3585600,"01dc06df35533f5c5978e53c190979263c77ac41b33aaa8cbc6dc5f4d1b1db254501912d34b4ffebe92f2b46416598d1533fcd193cc68ffc7c90ffc87642c2d022050c000001294c084af1c29464529f636c3964590ea09fbbfba71f7d1b3b04ad46b3eea8410185bedba855c962d4894bc3f6084ba54659dda62d546ca835f37624bf376000390000013c39072b6c15679ada23ca6a4c1ba73ed70064c785d4bdf74f26c280cf1b4f0200000001d280b7a861cf5346a0b0399071b4ba982d30b2d4d81efee9ee67aceed186ae4101cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3628800,"0110c3ccd928f8f586116261291f3bf767d31c1df5291f65e6e3509262d99e692501b365a800a4e9f75fd4c79131bedf527051fb89a21ba61eeae3b8ca530c6b022a0c000000000159d094918e63fbcdb4c7d11cba60b2c36867b30883885a749451de95805224280000016615ff1f969328b36726f9bd6e3f6e3563be459ba76c97bf2087329093f66e36000001d280b7a861cf5346a0b0399071b4ba982d30b2d4d81efee9ee67aceed186ae4101cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3672000,"01c7a42bc402f14e49436d7e58ac5f184d3ec8ef5d225a8646acf5a0f069540d16000c00011bf227bab6473b8a7de45666b4ee4e78b5661672023d0792cc514287d184fd0700000000000001b6c92cbdfebc410e0487eaaef818e8a1235e8769ac59f14a0ae57819a43c310e0001d280b7a861cf5346a0b0399071b4ba982d30b2d4d81efee9ee67aceed186ae4101cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3715200,"013b846812e6e8b7a8b287b0155a3402841bb29654a9e779344fb354bed36bcc13000c01a1b1f9cb0c43f37309712064acf453bc25cff51aa5e99e9c6ef170e3f1df0369000153df0a7b369557d791bcd974b3cb498d67a4765a89bdebb434a4e12738cea125018b98c3a4de1f68d9e34d07bbc195744c77484d33a76ae5250beb501ac915773000019cf1f62c39072b19d417e6e4231f855a506fa3135e4d641a09af9401a062270301ba70328e92baf57c6d764ba8d9490ab2a36166bd5869d8f355b8cad94b5f920f0001b6c92cbdfebc410e0487eaaef818e8a1235e8769ac59f14a0ae57819a43c310e0001d280b7a861cf5346a0b0399071b4ba982d30b2d4d81efee9ee67aceed186ae4101cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a"),
    (3758400,"01dad3163bb15cb6c084ee6b58062165c87d74bad920034e1f664387b860d4356d013b2d420f6fbb1dcd12fb420c74b5aa290e289e766b40995b518914685eecac460c015120e4c2ae792da3621cbb9638e5790881394f711385cede7034c9e05776eb1c0001a5fa713bde9aeb845b01587cfa740f65e4cfce86bc7ea7d9069525b9c4aaf84501fb96b56858d068fff8331a03bca0882dcde1b6468cbbe7de05eb01049c7bff6900014cb3792d12d26638787feae275e3d1738d4094ab9e57d8ce2fabd6a5f5344f2201b5a4a8f9964e892201a22d902383145ae88f39dcd1170568142d6c03d416091801551a70b1c86f61bb5f2189550009c5d6c1cc7507c3d9b267f7a7276d614b416201b6c92cbdfebc410e0487eaaef818e8a1235e8769ac59f14a0ae57819a43c310e0001d280b7a861cf5346a0b0399071b4ba982d30b2d4d81efee9ee67aceed186ae4101cfba1bf888eb87c36b723c3121dfddf9317af542f93237a26cce3e8f4fbc291a")];
    let used_checkpoints = if is_testnet {
        test_network_checkpoints.as_slice()
    } else {
        main_network_checkpoints.as_slice()
    };

    used_checkpoints
        .iter()
        .rev()
        .find(|x| x.0 < block_height)
        .copied()
        .unwrap_or(used_checkpoints[0])
}

//TODO: update once we have more checkpoints on testnet
#[cfg(test)]
mod test {
    use crate::checkpoint::get_checkpoint;
    use pivx_primitives::merkle_tree::CommitmentTree;
    use pivx_primitives::sapling::Node;
    use std::io::Cursor;
    #[test]
    fn check_testnet_checkpoints() {
        assert_eq!(get_checkpoint(11257770, true).0, 1125777);
    }
    #[test]
    fn check_mainnet_checkpoints() {
        assert_eq!(get_checkpoint(2700000 - 1, false).0, 2700000);
        assert_eq!(get_checkpoint(2700001, false).0, 2700000);
        assert_eq!(get_checkpoint(3758400 + 1, false).0, 3758400);
        assert_eq!(get_checkpoint((3758400 + 3715200) / 2, false).0, 3715200);
        let buff = Cursor::new(
            hex::decode(get_checkpoint(2700000, false).1)
                .expect("Cannot decode commitment tree from hexadecimal"),
        );
        let tree = CommitmentTree::<Node>::read(buff).expect("Cannot decode commitment tree!");
        assert_eq!(tree, CommitmentTree::<Node>::empty());
    }
}

//Output the closest checkpoint to a given blockheight
#[wasm_bindgen]
pub fn get_closest_checkpoint(block_height: i32, is_testnet: bool) -> JsValue {
    return serde_wasm_bindgen::to_value(&get_checkpoint(block_height, is_testnet))
        .expect("Cannot serialize checkpoint");
}

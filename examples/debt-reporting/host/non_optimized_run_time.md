# Speed notes
- The preflight is 4 minutes, and the guest execution on my local (no crazy gpu) mac only 5 seconds
- The constants have to be read onchain first but after that all the



## autoUSD

Starting debt reporting host
Starting Address Preflight
Inside of helper in the host!
autopool: 0xa7569a44f348d3d70d8ad5889e50f78e33d80d35
systemRegistry: 0x2218f90a98b0c070676f249ef44834686daa4285
rootPriceOracle: 0x61f8be7fd721e80c0249829eae6f0daf21bc2cac
baseAsset: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48
destinationVaultKeys len: 26
Finished Address Preflight in 170.103073667s
Starting Prices Preflight for block 24034282
Finished Prices Preflight for block 24034282 in 146.392357375s
Starting Prices Preflight for block 24034283
Finished Prices Preflight for block 24034283 in 147.589144291s
Starting Guest!
2025-12-17T19:08:56.951780Z  INFO risc0_zkvm::host::server::exec::executor: execution time: 9.988271667s
2025-12-17T19:08:56.951814Z  INFO risc0_zkvm::host::server::session: number of segments: 1109
2025-12-17T19:08:56.951817Z  INFO risc0_zkvm::host::server::session: 1161887744 total cycles
2025-12-17T19:08:56.951818Z  INFO risc0_zkvm::host::server::session: 963422246 user cycles (82.92%)
2025-12-17T19:08:56.951820Z  INFO risc0_zkvm::host::server::session: 166217797 paging cycles (14.31%)
2025-12-17T19:08:56.951822Z  INFO risc0_zkvm::host::server::session: 32247701 reserved cycles (2.78%)
2025-12-17T19:08:56.951823Z  INFO risc0_zkvm::host::server::session: ecalls
2025-12-17T19:08:56.951829Z  INFO risc0_zkvm::host::server::session:    493463 Sha2 calls, 36525374 cycles, (3.14%)
2025-12-17T19:08:56.951868Z  INFO risc0_zkvm::host::server::session:    1153374 Read calls, 4466582 cycles, (0.38%)
2025-12-17T19:08:56.951870Z  INFO risc0_zkvm::host::server::session:    1 Terminate calls, 2 cycles, (0.00%)
2025-12-17T19:08:56.951872Z  INFO risc0_zkvm::host::server::session:    0 Write calls, 0 cycles, (0.00%)
2025-12-17T19:08:56.951873Z  INFO risc0_zkvm::host::server::session:    0 User calls, 0 cycles, (0.00%)
2025-12-17T19:08:56.951874Z  INFO risc0_zkvm::host::server::session:    0 Poseidon2 calls, 0 cycles, (0.00%)
2025-12-17T19:08:56.951875Z  INFO risc0_zkvm::host::server::session:    0 BigInt calls, 0 cycles, (0.00%)
2025-12-17T19:08:56.951876Z  INFO risc0_zkvm::host::server::session: syscalls
2025-12-17T19:08:56.951878Z  INFO risc0_zkvm::host::server::session:    515019 Read calls
2025-12-17T19:08:56.951937Z  INFO risc0_zkvm::host::server::session:    61571 Keccak calls
2025-12-17T19:08:56.951939Z  INFO risc0_zkvm::host::server::session:    95 ProveKeccak calls
2025-12-17T19:08:56.951940Z  INFO risc0_zkvm::host::server::session:    1 Write calls
2025-12-17T19:08:56.951941Z  INFO risc0_zkvm::host::server::session:    1 VerifyIntegrity2 calls
2025-12-17T19:08:56.951942Z  INFO risc0_zkvm::host::server::session:    0 VerifyIntegrity calls
Guest execution took 10.429732s
Stub for submitting a transaction to validate debt reporting

Commitment: Commitment {
    version: "Block",
    id: 24034284,
    digest: 0x66d9d0a09f6893d07aef9bdc673b1dee9b3a38ed2e61abe0e280373cbbff2117,
    configID: 0x9a223c7ca04c969f1cacbe5b8db44c308b2c53390505d3d48c834ed4469fc839,
}
Autopool constants:
  autopool:       0xa7569a44f348d3d70d8ad5889e50f78e33d80d35
  systemRegistry: 0x2218f90a98b0c070676f249ef44834686daa4285
  rootPriceOracle:0x61f8be7fd721e80c0249829eae6f0daf21bc2cac
  baseAsset:      0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48
  destinationVaultKeys: 26
  priceInfo rows:        26

idx  token                                      pool                                       baseAsset                                  destinationVault                                            avgSpot               latestSafe spotSafe? 
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
0    0x9d39a5de30e57443bff2a8307a4256c8797a3497 0x9d39a5de30e57443bff2a8307a4256c8797a3497 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xc73af8064ecbfccbf687cf36a8abaff011eab1fc                  1210816                  1211051 false     
1    0x83f20f44975d03b1b09e64809b757c47f942beea 0x83f20f44975d03b1b09e64809b757c47f942beea 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x2c4b73c6a4c5edb8faca363f8e4c906475e19078                  1168694                  1169640 false     
2    0xa3931d71877c0e7a3148cb7eb4463524fec27fbd 0xa3931d71877c0e7a3148cb7eb4463524fec27fbd 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xe3200b14481916d35d18cdb3ba51044caa01813c                  1078790                  1079663 false     
3    0x0655977feb2f289a4ab78af67bab0d17aab84367 0x0655977feb2f289a4ab78af67bab0d17aab84367 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x8aca8accfb69adeff607431e0f25466b7b76a8ad                  1081716                  1082318 false     
4    0xcf62f905562626cfcdd2261162a51fd02fc9c5b6 0xcf62f905562626cfcdd2261162a51fd02fc9c5b6 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x9b18801c9e95f45ad0add54ef2eb4690c44d170d                  1173242                  1173821 false     
5    0x5b03cccab7ba3010fa5cad23746cbf0794938e96 0x5b03cccab7ba3010fa5cad23746cbf0794938e96 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xd7900d87069c815a299bda7afdcd7eee98fe4b6c                  1018259                  1018610 false     
6    0x4dece678ceceb27446b35c672dc7d61f30bad69e 0x4dece678ceceb27446b35c672dc7d61f30bad69e 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x65efcf2cce562dcbf07e805eebedef21dbd8ea3d                  1022002                  1022431 false     
7    0x390f3595bca2df7d23783dfd126427cceb997bf4 0x390f3595bca2df7d23783dfd126427cceb997bf4 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x7583b1589add33320366a48a92794d77763fae9e                  1022310                  1022757 false     
8    0x635ef0056a597d13863b73825cca297236578595 0x635ef0056a597d13863b73825cca297236578595 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xa345ceeccf8fe6ae33fe1d655b4806492251c2a8                  1009792                  1009757 false     
9    0x0cd6f267b2086bea681e922e19d40512511be538 0x0cd6f267b2086bea681e922e19d40512511be538 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x9906eb64ba32fb4fd3e5541eca95e57610084d02                  1002653                  1002873 false     
10   0x57064f49ad7123c92560882a45518374ad982e85 0x57064f49ad7123c92560882a45518374ad982e85 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xc099899d0278ce83976218cbe58d01dd382dca32                  1101386                  1102124 false     
11   0xd29f8980852c2c76fc3f6e96a7aa06e0bedcc1b1 0xd29f8980852c2c76fc3f6e96a7aa06e0bedcc1b1 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x53c8c211350b635269e02b2c2f1077b850c657db                  1087025                  1087523 false     
12   0x2bbe31d63e6813e3ac858c04dae43fb2a72b0d11 0x2bbe31d63e6813e3ac858c04dae43fb2a72b0d11 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x28d2f508127b0617ded8243836eb7b5787828648                  1081074                  1081721 false     
13   0xfd1627e3f3469c8392c8c3a261d8f0677586e5e1 0xfd1627e3f3469c8392c8c3a261d8f0677586e5e1 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x3ee3f7a0b244004b0678eba98ba091ec101513ee                  1069111                  1069111 false     
14   0x4f493b7de8aac7d55f71853688b1f7c8f0243c85 0x4f493b7de8aac7d55f71853688b1f7c8f0243c85 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xbb2d2dd491204a86ec10a1a6972f940b34fe060e                  1013099                  1013174 false     
15   0x81a2612f6dea269a6dd1f6deab45c5424ee2c4b7 0x81a2612f6dea269a6dd1f6deab45c5424ee2c4b7 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xb949c40f18fb52747d96850696c954718168fde3                  1023292                  1023372 false     
16   0x167478921b907422f8e88b43c4af2b8bea278d3a 0x167478921b907422f8e88b43c4af2b8bea278d3a 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xad7b8db1efee849c67060c6b10bc7d6d3c607aa5                  1179737                  1180522 false     
17   0x3cef1afc0e8324b57293a6e7ce663781bbefbb79 0x3cef1afc0e8324b57293a6e7ce663781bbefbb79 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x6d9b1cf3698fd5f2e2f5e6344a20a4c062960bc9                  1048125                  1048896 false     
18   0x9fb7b4477576fe5b32be4c1843afb1e55f251b33 0x9fb7b4477576fe5b32be4c1843afb1e55f251b33 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x7876f91bb22148345b3de16af9448081e9853830                  1178786                  1178829 false     
19   0x5c20b550819128074fd538edf79791733ccedd18 0x5c20b550819128074fd538edf79791733ccedd18 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xe4545f9dbc30ccb6cda6930ddfd69f3d419fcb61                  1173802                  1173694 false     
20   0x8eb67a509616cd6a7c1b3c8c21d48ff57df3d458 0x8eb67a509616cd6a7c1b3c8c21d48ff57df3d458 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x3f3565c5a2aa05f76830a5a51c2f32018de856a3                  1133596                  1133637 false     
21   0xdd0f28e19c1780eb6396170735d45153d261490d 0xdd0f28e19c1780eb6396170735d45153d261490d 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x0f170e37e5e0c617148517b831a49292c38c363d                  1101331                  1101370 false     
22   0xd4fa2d31b7968e448877f69a96de69f5de8cd23e 0xd4fa2d31b7968e448877f69a96de69f5de8cd23e 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x769c6bea7db25f4c0a52f0db0960c42f708dbf41                  1156142                  1156184 false     
23   0x7bc3485026ac48b6cf9baf0a377477fff5703af8 0x7bc3485026ac48b6cf9baf0a377477fff5703af8 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xd6e262094ffa407068b0bef6bdbcd16db982b59e                  1150409                  1150303 false     
24   0xbeef01735c132ada46aa9aa4c54623caa92a64cb 0xbeef01735c132ada46aa9aa4c54623caa92a64cb 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0xc5c95fcad37e466e25e6eca1977bbf75c0e1004a                  1111267                  1111307 false     
25   0x85b2b559bc2d21104c4defdd6efca8a20343361d 0x85b2b559bc2d21104c4defdd6efca8a20343361d 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 0x366c094c5563cd12af27b9afff2200b0e0d056e0                  1033976                  1033996 false     

(Stub) would now build + send tx with proof + commitment payload...


### autoETH


Starting debt reporting host
Starting Address Preflight
Inside of helper in the host!
autopool: 0x0a2b94f6871c1d7a32fe58e1ab5e6dea2f114e56
systemRegistry: 0x2218f90a98b0c070676f249ef44834686daa4285
rootPriceOracle: 0x61f8be7fd721e80c0249829eae6f0daf21bc2cac
baseAsset: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
destinationVaultKeys len: 12
Finished Address Preflight in 81.801258417s
Starting Prices Preflight for block 24034251
Finished Prices Preflight for block 24034251 in 66.051742s
Starting Prices Preflight for block 24034252
Finished Prices Preflight for block 24034252 in 65.886370542s
Starting Guest!
2025-12-17T18:58:30.337103Z  INFO risc0_zkvm::host::server::exec::executor: execution time: 4.088231458s
2025-12-17T18:58:30.337132Z  INFO risc0_zkvm::host::server::session: number of segments: 449
2025-12-17T18:58:30.337134Z  INFO risc0_zkvm::host::server::session: 470286336 total cycles
2025-12-17T18:58:30.337135Z  INFO risc0_zkvm::host::server::session: 391652688 user cycles (83.28%)
2025-12-17T18:58:30.337137Z  INFO risc0_zkvm::host::server::session: 65429750 paging cycles (13.91%)
2025-12-17T18:58:30.337138Z  INFO risc0_zkvm::host::server::session: 13203898 reserved cycles (2.81%)
2025-12-17T18:58:30.337140Z  INFO risc0_zkvm::host::server::session: ecalls
2025-12-17T18:58:30.337145Z  INFO risc0_zkvm::host::server::session:    315718 Sha2 calls, 23367484 cycles, (4.97%)
2025-12-17T18:58:30.337167Z  INFO risc0_zkvm::host::server::session:    696818 Read calls, 2776790 cycles, (0.59%)
2025-12-17T18:58:30.337170Z  INFO risc0_zkvm::host::server::session:    1 Terminate calls, 2 cycles, (0.00%)
2025-12-17T18:58:30.337171Z  INFO risc0_zkvm::host::server::session:    0 Write calls, 0 cycles, (0.00%)
2025-12-17T18:58:30.337172Z  INFO risc0_zkvm::host::server::session:    0 User calls, 0 cycles, (0.00%)
2025-12-17T18:58:30.337173Z  INFO risc0_zkvm::host::server::session:    0 Poseidon2 calls, 0 cycles, (0.00%)
2025-12-17T18:58:30.337174Z  INFO risc0_zkvm::host::server::session:    0 BigInt calls, 0 cycles, (0.00%)
2025-12-17T18:58:30.337175Z  INFO risc0_zkvm::host::server::session: syscalls
2025-12-17T18:58:30.337177Z  INFO risc0_zkvm::host::server::session:    308954 Read calls
2025-12-17T18:58:30.337224Z  INFO risc0_zkvm::host::server::session:    39392 Keccak calls
2025-12-17T18:58:30.337226Z  INFO risc0_zkvm::host::server::session:    61 ProveKeccak calls
2025-12-17T18:58:30.337227Z  INFO risc0_zkvm::host::server::session:    1 Write calls
2025-12-17T18:58:30.337228Z  INFO risc0_zkvm::host::server::session:    1 VerifyIntegrity2 calls
2025-12-17T18:58:30.337229Z  INFO risc0_zkvm::host::server::session:    0 VerifyIntegrity calls
Guest execution took 4.456709917s
Stub for submitting a transaction to validate debt reporting

Commitment: Commitment {
    version: "Block",
    id: 24034253,
    digest: 0xde395a4d52a12489585c55cefe80369c7e0a30f81622eceb1c877c7679b7e522,
    configID: 0x9a223c7ca04c969f1cacbe5b8db44c308b2c53390505d3d48c834ed4469fc839,
}
Autopool constants:
  autopool:       0x0a2b94f6871c1d7a32fe58e1ab5e6dea2f114e56
  systemRegistry: 0x2218f90a98b0c070676f249ef44834686daa4285
  rootPriceOracle:0x61f8be7fd721e80c0249829eae6f0daf21bc2cac
  baseAsset:      0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
  destinationVaultKeys: 12
  priceInfo rows:        12

idx  token                                      pool                                       baseAsset                                  destinationVault                                            avgSpot               latestSafe spotSafe? 
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
0    0x57c23c58b1d8c3292c15becf07c62c5c52457a42 0x57c23c58b1d8c3292c15becf07c62c5c52457a42 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0x87e25ffe5c3a2720cd43f5eb8ec41ac0ce699d07      1017113402257386155      1016873083207376364 false     
1    0x59ab5a5b5d617e478a2479b0cad80da7e2831492 0x59ab5a5b5d617e478a2479b0cad80da7e2831492 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0x1ea622fa030e4a78f4cc2f305dd3c08da3f08573      1052712406589458016      1052720194587215598 false     
2    0x6951bdc4734b9f7f3e1b74afebc670c736a0edb6 0x6951bdc4734b9f7f3e1b74afebc670c736a0edb6 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0x2e0e2ab6505a1182367dfb1e3c66648bf3eea020      1019141644214943823      1019289581237946002 false     
3    0xc8eb2cf2f792f77af0cd9e203305a585e588179d 0xc8eb2cf2f792f77af0cd9e203305a585e588179d 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0xba1462f43c6f60ebd1c62735c94e428ad073e01a       998672288596375242       998636226553530449 false     
4    0x6951bdc4734b9f7f3e1b74afebc670c736a0edb6 0x6951bdc4734b9f7f3e1b74afebc670c736a0edb6 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0xe4433d00cf48bfe0c672d9949f2cd2c008bffc04      1019141644214943823      1019289581237946002 false     
5    0xdb74dfdd3bb46be8ce6c33dc9d82777bcfc3ded5 0xdb74dfdd3bb46be8ce6c33dc9d82777bcfc3ded5 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0x5c6aeb9ef0d5bba4e6691f381003503fd0d45126      1062824052398088701      1062535086058188136 false     
6    0x7f39c581f595b53c5cb19bd0b3f8da6c935e2ca0 0x7f39c581f595b53c5cb19bd0b3f8da6c935e2ca0 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0xd100c932801390fdebce11f26f611d4898b44236       646790319157244723      1221458445154686690 true      
7    0xe080027bd47353b5d1639772b4a75e9ed3658a0d 0xe080027bd47353b5d1639772b4a75e9ed3658a0d 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0x3f55eedde51504e6ed0ec30e8289b4da11edb7f9      1055702953079428741      1055896367839374782 false     
8    0xc8eb2cf2f792f77af0cd9e203305a585e588179d 0xc8eb2cf2f792f77af0cd9e203305a585e588179d 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0x43d99d04985ef2231f7d9b5d9111d2189d9fd971       998672288596375242       998636226553530449 false     
9    0x6b31a94029fd7840d780191b6d63fa0d269bd883 0x6b31a94029fd7840d780191b6d63fa0d269bd883 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0x9b163e15121816be53f8d5c85fbefd6e6d9bebcd      1018542629686454286      1018535190171852684 false     
10   0x57c23c58b1d8c3292c15becf07c62c5c52457a42 0x57c23c58b1d8c3292c15becf07c62c5c52457a42 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0x4142e3a17391676c66ddf1285e43889f168ee237      1017113402257386155      1016873083207376364 false     
11   0xa0d3707c569ff8c87fa923d3823ec5d81c98be78 0xa0d3707c569ff8c87fa923d3823ec5d81c98be78 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 0x2c7120dccf1c14a37a26a4955475d45d34a3d7e7      1194035832423163304      1193580406556753209 false     

(Stub) would now build + send tx with proof + commitment payload...
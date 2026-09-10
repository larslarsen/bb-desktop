# WAL-013 temporary native initialization diagnostic; restored

Actual bounded execution; inherited release blockers remain.

```json
{
  "commands": [
    {
      "argv": [
        "hermes",
        "--version"
      ],
      "exit": 0,
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n"
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "b7314a346e2b9b8a202542cf75422142e6d1e397\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/fixtures/wallet-broker/x11-window.py\n M test/securityPolicy.node.js\n M test/walletAccountManagement.node.js\n M test/walletAccountWindowSmoke.node.js\n M test/walletStartup.node.js\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md\n?? docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md\n"
    },
    {
      "argv": [
        "node",
        "scripts/build-wallet-broker.js"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.80s\n"
    },
    {
      "argv": [
        "node",
        "-e",
        "const cp=require('child_process'); const original=cp.spawn; const begin=Date.now(); let seq=0; const log=(label,value)=>process.stdout.write(JSON.stringify({ms:Date.now()-begin,label,value})+'\\n'); cp.spawn=function(file,args,options){ const child=original.apply(this,arguments); const id=++seq; const broker=file.endsWith('/bitbook-wallet-broker'); const inspect=args&&args[args.length-1]==='inspect'; log('spawn',{id,pid:child.pid,broker,inspect}); let bytes=0; if(broker&&child.stderr)child.stderr.on('data',data=>{bytes+=data.length;if(bytes<32768)log('broker_stderr',String(data));}); if(inspect&&child.stdout)child.stdout.on('data',data=>log('windows',String(data))); child.once('close',(code,signal)=>log('close',{id,code,signal})); return child; }; require('./test/walletAccountWindowSmoke.node.js').tests[0].fn().then(()=>log('result','passed'),error=>{log('failure',String(error.stack||error));process.exitCode=1;});"
      ],
      "exit": 1,
      "output": "{\"ms\":1146,\"label\":\"spawn\",\"value\":{\"id\":1,\"pid\":4143723,\"broker\":true,\"inspect\":false}}\n{\"ms\":1203,\"label\":\"spawn\",\"value\":{\"id\":2,\"pid\":4143726,\"broker\":false,\"inspect\":true}}\n{\"ms\":1223,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":1228,\"label\":\"close\",\"value\":{\"id\":2,\"code\":0,\"signal\":null}}\n{\"ms\":1378,\"label\":\"spawn\",\"value\":{\"id\":3,\"pid\":4143727,\"broker\":false,\"inspect\":true}}\n{\"ms\":1399,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":1402,\"label\":\"close\",\"value\":{\"id\":3,\"code\":0,\"signal\":null}}\n{\"ms\":1525,\"label\":\"spawn\",\"value\":{\"id\":4,\"pid\":4143728,\"broker\":false,\"inspect\":true}}\n{\"ms\":1545,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":1548,\"label\":\"close\",\"value\":{\"id\":4,\"code\":0,\"signal\":null}}\n{\"ms\":1679,\"label\":\"spawn\",\"value\":{\"id\":5,\"pid\":4143729,\"broker\":false,\"inspect\":true}}\n{\"ms\":1699,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":1702,\"label\":\"close\",\"value\":{\"id\":5,\"code\":0,\"signal\":null}}\n{\"ms\":1829,\"label\":\"spawn\",\"value\":{\"id\":6,\"pid\":4143730,\"broker\":false,\"inspect\":true}}\n{\"ms\":1849,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":1853,\"label\":\"close\",\"value\":{\"id\":6,\"code\":0,\"signal\":null}}\n{\"ms\":1979,\"label\":\"spawn\",\"value\":{\"id\":7,\"pid\":4143731,\"broker\":false,\"inspect\":true}}\n{\"ms\":1999,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":2002,\"label\":\"close\",\"value\":{\"id\":7,\"code\":0,\"signal\":null}}\n{\"ms\":2128,\"label\":\"spawn\",\"value\":{\"id\":8,\"pid\":4143732,\"broker\":false,\"inspect\":true}}\n{\"ms\":2148,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":2151,\"label\":\"close\",\"value\":{\"id\":8,\"code\":0,\"signal\":null}}\n{\"ms\":2173,\"label\":\"broker_stderr\",\"value\":\"WAL013_NATIVE_INIT: glutin error: [a5] GLXBadFBConfig\\n\"}\n{\"ms\":2174,\"label\":\"broker_stderr\",\"value\":\"broker closed\\n\"}\n{\"ms\":2177,\"label\":\"close\",\"value\":{\"id\":1,\"code\":1,\"signal\":null}}\n{\"ms\":2277,\"label\":\"spawn\",\"value\":{\"id\":9,\"pid\":4143733,\"broker\":false,\"inspect\":true}}\n{\"ms\":2297,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":2301,\"label\":\"close\",\"value\":{\"id\":9,\"code\":0,\"signal\":null}}\n{\"ms\":2431,\"label\":\"spawn\",\"value\":{\"id\":10,\"pid\":4143734,\"broker\":false,\"inspect\":true}}\n{\"ms\":2452,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":2456,\"label\":\"close\",\"value\":{\"id\":10,\"code\":0,\"signal\":null}}\n{\"ms\":2611,\"label\":\"spawn\",\"value\":{\"id\":11,\"pid\":4143735,\"broker\":false,\"inspect\":true}}\n{\"ms\":2632,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":2635,\"label\":\"close\",\"value\":{\"id\":11,\"code\":0,\"signal\":null}}\n{\"ms\":2765,\"label\":\"spawn\",\"value\":{\"id\":12,\"pid\":4143736,\"broker\":false,\"inspect\":true}}\n{\"ms\":2785,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":2789,\"label\":\"close\",\"value\":{\"id\":12,\"code\":0,\"signal\":null}}\n{\"ms\":2919,\"label\":\"spawn\",\"value\":{\"id\":13,\"pid\":4143737,\"broker\":false,\"inspect\":true}}\n{\"ms\":2941,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":2945,\"label\":\"close\",\"value\":{\"id\":13,\"code\":0,\"signal\":null}}\n{\"ms\":3097,\"label\":\"spawn\",\"value\":{\"id\":14,\"pid\":4143738,\"broker\":false,\"inspect\":true}}\n{\"ms\":3118,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":3122,\"label\":\"close\",\"value\":{\"id\":14,\"code\":0,\"signal\":null}}\n{\"ms\":3276,\"label\":\"spawn\",\"value\":{\"id\":15,\"pid\":4143739,\"broker\":false,\"inspect\":true}}\n{\"ms\":3296,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":3300,\"label\":\"close\",\"value\":{\"id\":15,\"code\":0,\"signal\":null}}\n{\"ms\":3430,\"label\":\"spawn\",\"value\":{\"id\":16,\"pid\":4143740,\"broker\":false,\"inspect\":true}}\n{\"ms\":3450,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":3454,\"label\":\"close\",\"value\":{\"id\":16,\"code\":0,\"signal\":null}}\n{\"ms\":3581,\"label\":\"spawn\",\"value\":{\"id\":17,\"pid\":4143742,\"broker\":false,\"inspect\":true}}\n{\"ms\":3600,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":3604,\"label\":\"close\",\"value\":{\"id\":17,\"code\":0,\"signal\":null}}\n{\"ms\":3733,\"label\":\"spawn\",\"value\":{\"id\":18,\"pid\":4143743,\"broker\":false,\"inspect\":true}}\n{\"ms\":3752,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":3755,\"label\":\"close\",\"value\":{\"id\":18,\"code\":0,\"signal\":null}}\n{\"ms\":3882,\"label\":\"spawn\",\"value\":{\"id\":19,\"pid\":4143744,\"broker\":false,\"inspect\":true}}\n{\"ms\":3901,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":3904,\"label\":\"close\",\"value\":{\"id\":19,\"code\":0,\"signal\":null}}\n{\"ms\":4033,\"label\":\"spawn\",\"value\":{\"id\":20,\"pid\":4143745,\"broker\":false,\"inspect\":true}}\n{\"ms\":4052,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":4056,\"label\":\"close\",\"value\":{\"id\":20,\"code\":0,\"signal\":null}}\n{\"ms\":4184,\"label\":\"spawn\",\"value\":{\"id\":21,\"pid\":4143747,\"broker\":false,\"inspect\":true}}\n{\"ms\":4203,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":4207,\"label\":\"close\",\"value\":{\"id\":21,\"code\":0,\"signal\":null}}\n{\"ms\":4335,\"label\":\"spawn\",\"value\":{\"id\":22,\"pid\":4143748,\"broker\":false,\"inspect\":true}}\n{\"ms\":4354,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":4358,\"label\":\"close\",\"value\":{\"id\":22,\"code\":0,\"signal\":null}}\n{\"ms\":4488,\"label\":\"spawn\",\"value\":{\"id\":23,\"pid\":4143749,\"broker\":false,\"inspect\":true}}\n{\"ms\":4509,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":4513,\"label\":\"close\",\"value\":{\"id\":23,\"code\":0,\"signal\":null}}\n{\"ms\":4669,\"label\":\"spawn\",\"value\":{\"id\":24,\"pid\":4143750,\"broker\":false,\"inspect\":true}}\n{\"ms\":4690,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":4694,\"label\":\"close\",\"value\":{\"id\":24,\"code\":0,\"signal\":null}}\n{\"ms\":4843,\"label\":\"spawn\",\"value\":{\"id\":25,\"pid\":4143751,\"broker\":false,\"inspect\":true}}\n{\"ms\":4862,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":4865,\"label\":\"close\",\"value\":{\"id\":25,\"code\":0,\"signal\":null}}\n{\"ms\":4994,\"label\":\"spawn\",\"value\":{\"id\":26,\"pid\":4143752,\"broker\":false,\"inspect\":true}}\n{\"ms\":5013,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":5017,\"label\":\"close\",\"value\":{\"id\":26,\"code\":0,\"signal\":null}}\n{\"ms\":5141,\"label\":\"spawn\",\"value\":{\"id\":27,\"pid\":4143753,\"broker\":false,\"inspect\":true}}\n{\"ms\":5161,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":5164,\"label\":\"close\",\"value\":{\"id\":27,\"code\":0,\"signal\":null}}\n{\"ms\":5292,\"label\":\"spawn\",\"value\":{\"id\":28,\"pid\":4143754,\"broker\":false,\"inspect\":true}}\n{\"ms\":5312,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":5315,\"label\":\"close\",\"value\":{\"id\":28,\"code\":0,\"signal\":null}}\n{\"ms\":5444,\"label\":\"spawn\",\"value\":{\"id\":29,\"pid\":4143755,\"broker\":false,\"inspect\":true}}\n{\"ms\":5465,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":5468,\"label\":\"close\",\"value\":{\"id\":29,\"code\":0,\"signal\":null}}\n{\"ms\":5596,\"label\":\"spawn\",\"value\":{\"id\":30,\"pid\":4143756,\"broker\":false,\"inspect\":true}}\n{\"ms\":5617,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":5620,\"label\":\"close\",\"value\":{\"id\":30,\"code\":0,\"signal\":null}}\n{\"ms\":5749,\"label\":\"spawn\",\"value\":{\"id\":31,\"pid\":4143757,\"broker\":false,\"inspect\":true}}\n{\"ms\":5770,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":5774,\"label\":\"close\",\"value\":{\"id\":31,\"code\":0,\"signal\":null}}\n{\"ms\":5925,\"label\":\"spawn\",\"value\":{\"id\":32,\"pid\":4143758,\"broker\":false,\"inspect\":true}}\n{\"ms\":5946,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":5949,\"label\":\"close\",\"value\":{\"id\":32,\"code\":0,\"signal\":null}}\n{\"ms\":6076,\"label\":\"spawn\",\"value\":{\"id\":33,\"pid\":4143759,\"broker\":false,\"inspect\":true}}\n{\"ms\":6096,\"label\":\"windows\",\"value\":\"{\\\"windows\\\":[]}\\n\"}\n{\"ms\":6099,\"label\":\"close\",\"value\":{\"id\":33,\"code\":0,\"signal\":null}}\n{\"ms\":6176,\"label\":\"failure\",\"value\":\"Error: native account window did not initialize hidden\\n    at waitForHiddenInitializedWindow (<repo>/test/walletAccountWindowSmoke.node.js:495:31)\\n    at async Object.fn (<repo>/test/walletAccountWindowSmoke.node.js:728:5)\"}\n"
    },
    {
      "argv": [
        "node",
        "scripts/build-wallet-broker.js"
      ],
      "exit": 0,
      "output": "   Compiling bitbook-wallet-broker v0.1.0 (<repo>/wallet-broker)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.58s\n"
    }
  ],
  "accepted": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 67764dc0 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "b7314a346e2b9b8a202542cf75422142e6d1e397",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/fixtures/wallet-broker/x11-window.py\n M test/securityPolicy.node.js\n M test/walletAccountManagement.node.js\n M test/walletAccountWindowSmoke.node.js\n M test/walletStartup.node.js\n M wallet-broker/supervisor.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md\n?? docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md\n?? docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md",
  "session": {
    "id": "20260910_124632_7ad1a2",
    "model": "meituan/longcat-2.0:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "wallet-broker/supervisor.js": "57811beaebdd588f3328925d9b1d1b1d016100bfe1a57000a4ecccba57d60ee0",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "307480c4620c704af6f5a6f96afda0bf8941d5ee1a7583f6683dc30be0bb62bb",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
    "test/walletBrokerBuild.node.js": "b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de",
    "test/walletSupervisorShutdown.node.js": "f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/electronSecurity.node.js": "d70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "wallet-broker/src/accounts.rs": "f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1",
    "wallet-broker/tests/account_management.rs": "7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8",
    "wallet-broker/tests/account_native_ui.rs": "95584435ad547752481bfb2ebefbb83d734a6a0edbffb63a4ef8ec0681d166d5",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/account_ui.rs": "f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "5661f8945b7cebc331e4a2a1feb290d9ab0b2e77f4979a9adcaf2ac6cb3cdc94",
    "test/fixtures/wallet-broker/x11-window.py": "6a049e8dbcd10522d6a7c65f46792afac2c980aa33a413041e118a9d964aa25b",
    "wallet-broker/target/wal011-runtime-package-red-01.json": "5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd"
  },
  "diagnostic_only": true,
  "final_hashes": {
    "social-main.js": "d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a",
    "wallet-broker/supervisor.js": "57811beaebdd588f3328925d9b1d1b1d016100bfe1a57000a4ecccba57d60ee0",
    "scripts/build-wallet-broker.js": "4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445",
    "wallet-broker/src/runtime.rs": "6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962",
    "test/walletAccountManagement.node.js": "307480c4620c704af6f5a6f96afda0bf8941d5ee1a7583f6683dc30be0bb62bb",
    "test/walletStartup.node.js": "448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1",
    "test/walletBrokerBuild.node.js": "b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de",
    "test/walletSupervisorShutdown.node.js": "f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef",
    "test/walletSupervisorTransport.node.js": "e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700",
    "test/walletSupervisor.node.js": "7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4",
    "test/electronSecurity.node.js": "d70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "test/walletBrokerLaunchConfig.node.js": "90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "wallet-broker/launch-config.js": "68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8",
    "package.json": "76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5",
    "package-lock.json": "0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8",
    "scripts/security-policy.js": "fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078",
    "test/securityPolicy.node.js": "a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c",
    "wallet-broker/src/accounts.rs": "f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1",
    "wallet-broker/tests/account_management.rs": "7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8",
    "wallet-broker/tests/account_native_ui.rs": "95584435ad547752481bfb2ebefbb83d734a6a0edbffb63a4ef8ec0681d166d5",
    "wallet-broker/src/lib.rs": "e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437",
    "wallet-broker/Cargo.toml": "73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503",
    "wallet-broker/Cargo.lock": "b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71",
    "wallet-broker/src/vault.rs": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
    "wallet-broker/src/store.rs": "611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236",
    "wallet-broker/src/session.rs": "42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227",
    "wallet-broker/src/native.rs": "a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5",
    "wallet-broker/src/native_ui.rs": "c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f",
    "wallet-broker/src/account_ui.rs": "f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7",
    "wallet-broker/src/native_ui/zec_native_app_tests.rs": "d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0",
    "wallet-broker/src/native_ui/zec_review_tests.rs": "2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf",
    "test/walletBrokerRuntime.node.js": "989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592",
    "test/walletStartupSmoke.node.js": "a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef",
    "test/walletAccountWindowSmoke.node.js": "5661f8945b7cebc331e4a2a1feb290d9ab0b2e77f4979a9adcaf2ac6cb3cdc94",
    "test/fixtures/wallet-broker/x11-window.py": "6a049e8dbcd10522d6a7c65f46792afac2c980aa33a413041e118a9d964aa25b",
    "wallet-broker/target/wal011-runtime-package-red-01.json": "5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd"
  }
}
```

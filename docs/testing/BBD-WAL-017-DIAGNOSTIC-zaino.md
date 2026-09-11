# WAL017 bounded public RPC diagnostic

```json
{
  "session": [
    "20260911_090618_2a3ea4",
    "meituan/longcat-2.0:free",
    "nous"
  ],
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) · upstream ad03f20d · local 10b6d1a9 (+1 carried commit)",
  "endpoint": "https://zaino.testnet.unsafe.zec.rocks:443",
  "commands": [
    {
      "method": "GetLightdInfo",
      "label": "info",
      "elapsed_ms": 430,
      "http_status": 200,
      "grpc_status": "0",
      "bytes": 123,
      "error": null
    },
    {
      "method": "GetLatestBlock",
      "label": "tip",
      "elapsed_ms": 157,
      "http_status": 200,
      "grpc_status": "0",
      "bytes": 44,
      "error": null
    },
    {
      "method": "GetTreeState",
      "label": "prior",
      "elapsed_ms": 173,
      "http_status": 200,
      "grpc_status": "0",
      "bytes": 2477,
      "error": null
    },
    {
      "method": "GetBlockRange",
      "label": "next100",
      "elapsed_ms": 2001,
      "http_status": 200,
      "grpc_status": "0",
      "bytes": 1801018,
      "error": null
    },
    {
      "method": "GetTreeState",
      "label": "end",
      "elapsed_ms": 155,
      "http_status": 200,
      "grpc_status": "0",
      "bytes": 2797,
      "error": null
    }
  ]
}
```

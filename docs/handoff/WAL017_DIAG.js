'use strict';
// Reviewer-bounded public RPC diagnostic; never opens a wallet or accepts private input.
const fs = require('node:fs');
const path = require('node:path');
const http2 = require('node:http2');
const {spawnSync} = require('node:child_process');
const root = 'wallet-broker/target/wal017-diagnostic';
const endpoint = 'https://testnet.zec.rocks:443';
const prefix = '/cash.z.wallet.sdk.rpc.CompactTxStreamer/';
const limit = 20 * 1024 * 1024;
function varint(value) {
  const out=[];
  do {out.push((value & 127) | (value > 127 ? 128 : 0)); value=Math.floor(value/128);} while(value);
  return Buffer.from(out);
}
function blockId(height) {return Buffer.concat([Buffer.from([8]),varint(height)]);}
function nested(tag, value) {return Buffer.concat([Buffer.from([tag]),varint(value.length),value]);}
function rpc(method,body,label) {
  return new Promise(resolve => {
    const started=Date.now();const client=http2.connect(endpoint);
    let request;let done=false;let bytes=0;const chunks=[];let status=null;let httpStatus=null;
    const finish=(error=null)=>{
      if(done)return;done=true;clearTimeout(timer);
      if(request)request.close();client.destroy();
      if(!error)fs.writeFileSync(path.join(root,label+'.grpc'),Buffer.concat(chunks),{flag:'wx',mode:0o600});
      const result={method,label,elapsed_ms:Date.now()-started,http_status:httpStatus,grpc_status:status,bytes,error};
      console.log(JSON.stringify(result));resolve(result);
    };
    const timer=setTimeout(()=>finish('DEADLINE'),15000);
    client.on('error',()=>finish('CONNECTION'));
    client.on('connect',()=>{
      request=client.request({':method':'POST',':path':prefix+method,'content-type':'application/grpc','te':'trailers','grpc-timeout':'15S'});
      request.on('response',h=>{httpStatus=h[':status'];if(h['grpc-status']!==undefined)status=h['grpc-status'];});
      request.on('trailers',h=>{if(h['grpc-status']!==undefined)status=h['grpc-status'];});
      request.on('data',chunk=>{bytes+=chunk.length;if(bytes>limit)finish('LIMIT');else chunks.push(chunk);});
      request.on('error',()=>finish('STREAM'));
      request.on('end',()=>finish());
      const header=Buffer.alloc(5);header.writeUInt32BE(body.length,1);request.end(Buffer.concat([header,body]));
    });
  });
}
(async()=>{
  if(fs.existsSync(root))throw new Error('owned diagnostic directory already exists');
  const sid=process.env.HERMES_SESSION_ID;if(!sid)throw new Error('missing runner session');
  const metadata=spawnSync('python3',['-c',"from pathlib import Path;import sqlite3,json,os;db=sqlite3.connect((Path.home()/'.hermes/state.db').as_uri()+'?mode=ro',uri=True);print(json.dumps(db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?',(os.environ['HERMES_SESSION_ID'],)).fetchone()))"],{encoding:'utf8'});
  if(metadata.status!==0)throw new Error('metadata unavailable');
  const version=spawnSync('hermes',['--version'],{encoding:'utf8'});if(version.status!==0)throw new Error('version unavailable');
  fs.mkdirSync(root,{mode:0o700});
  const record={session:JSON.parse(metadata.stdout),version:version.stdout.split('\n')[0],endpoint,commands:[]};
  for(const [method,body,label] of [
    ['GetLightdInfo',Buffer.alloc(0),'info'],
    ['GetLatestBlock',Buffer.alloc(0),'tip'],
    ['GetTreeState',blockId(4308219),'prior'],
    ['GetBlockRange',Buffer.concat([nested(10,blockId(4308220)),nested(18,blockId(4308319))]),'next100'],
    ['GetTreeState',blockId(4308319),'end'],
  ])record.commands.push(await rpc(method,body,label));
  fs.writeFileSync(path.join(root,'summary.json'),JSON.stringify(record,null,2)+'\n',{flag:'wx',mode:0o600});
  fs.writeFileSync('docs/testing/BBD-WAL-017-DIAGNOSTIC-01.md','# WAL017 bounded public RPC diagnostic\n\n```json\n'+JSON.stringify(record,null,2)+'\n```\n',{flag:'wx'});
})().catch(()=>{console.error('Diagnostic stopped');process.exitCode=1;});

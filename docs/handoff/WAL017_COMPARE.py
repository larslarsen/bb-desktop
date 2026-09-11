from pathlib import Path
import io,json
root=Path('wallet-broker/target')
def varint(stream):
    value=0
    for shift in range(0,70,7):
        b=stream.read(1);assert len(b)==1
        value|=(b[0]&127)<<shift
        if b[0]<128:return value
    raise ValueError('oversized public protobuf integer')
def fields(data):
    stream=io.BytesIO(data);out={}
    while stream.tell()<len(data):
        tag=varint(stream);wire=tag&7
        if wire==0:value=varint(stream)
        elif wire==2:
            length=varint(stream);value=stream.read(length);assert len(value)==length
        else:raise ValueError('unexpected public protobuf wire type')
        out.setdefault(tag>>3,[]).append(value)
    return out
def frames(directory,name):
    data=(directory/(name+'.grpc')).read_bytes();assert len(data)<20*1024*1024
    stream=io.BytesIO(data);out=[]
    while header:=stream.read(5):
        assert len(header)==5 and header[0]==0
        length=int.from_bytes(header[1:],'big');body=stream.read(length);assert len(body)==length
        out.append(fields(body))
    return out
def tree_size(encoded):
    tree=io.BytesIO(bytes.fromhex(encoded.decode()))
    def node():
        b=tree.read(1);assert b in (b'\0',b'\1')
        if b==b'\1':assert len(tree.read(32))==32
        return int(b==b'\1')
    left=node();right=node();assert left or not right
    count=tree.read(1);assert len(count)==1 and count[0]<=31
    size=left+right+sum(node()*(1<<(i+1)) for i in range(count[0]))
    assert tree.read()==b''
    return size
observations=[]
for label in ['wal017-diagnostic','wal017-diagnostic-sameconnection','wal017-diagnostic-zaino']:
    directory=root/label
    summary=json.loads((directory/'summary.json').read_text())
    assert len(summary['commands'])==5 and all(c['grpc_status']=='0' and c['http_status']==200 and c['error'] is None for c in summary['commands'])
    blocks=frames(directory,'next100');prior=frames(directory,'prior')[0];end=frames(directory,'end')[0]
    info=frames(directory,'info')[0];tip=frames(directory,'tip')[0]
    assert len(blocks)==100 and [b[2][0] for b in blocks]==list(range(4308220,4308320))
    assert prior[1]==end[1]==[b'test'] and prior[2]==[4308219] and end[2]==[4308319]
    assert info[4]==[b'test'] and info[5]==[280000] and info[6]==[b'37a5165b'] and info[7]==tip[1]
    previous=bytes.fromhex(prior[3][0].decode())[::-1]
    sizes=[tree_size(prior[k][0]) for k in [5,6,7]]
    for block in blocks:
        assert block[4][0]==previous and len(block[3][0])==32
        counts=[0,0,0]
        for raw in block.get(7,[]):
            tx=fields(raw);assert len(tx[2][0])==32
            for i,k in enumerate([5,6,9]):counts[i]+=len(tx.get(k,[]))
        metadata=fields(block[8][0]);next_sizes=[metadata.get(k,[0])[0] for k in [1,2,3]]
        assert next_sizes==[sizes[i]+counts[i] for i in range(3)]
        previous=block[3][0];sizes=next_sizes
    end_sizes=[tree_size(end[k][0]) for k in [5,6,7]]
    matches=bytes.fromhex(end[3][0].decode())[::-1]==previous
    expected=label.endswith('zaino')
    assert matches==expected and (end_sizes==sizes)==expected
    observations.append({'endpoint':summary['endpoint'],'persistent_connection':label.endswith('sameconnection'),'range':[4308220,4308319],'prior_hash_matches':True,'end_hash_matches':matches,'compact_tree_sizes':sizes,'endpoint_tree_sizes':end_sizes})
assert (root/'wal017-diagnostic/prior.grpc').read_bytes()==(root/'wal017-diagnostic-zaino/prior.grpc').read_bytes()
print(json.dumps({'verified':True,'observations':observations,'product_changes':False,'user_wallet_modified':False},indent=2))

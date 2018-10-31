#!/usr/bin/env python
from subprocess import check_call

targets = [
    'x86_64-apple-darwin',
]

def sh(*command):
    print('> %s' % ' '.join(command))
    check_call(command)

for target in targets:
    # sh('rustup', 'target', 'add', target)
    sh('cargo', 'build', '--release', '--target', target)
    sh('mkdir', '-p', './hedera-sdk-go/libs/%s' % target)
    sh('cp', 'target/%s/release/libhedera.a' % target, './hedera-sdk-go/libs/%s/libhedera.a' % target)

@Library('jenkins_rust') _
rustPipeline(
    enableBenchmarks: false,
    osList: ['linux', 'win', 'osx', 'freebsd', 'web'],
    rustVersion: 'stable',
    buildArgs: '--release',
    artifactPatterns: ['target/release/*', 'target/criterion/**/*']
)

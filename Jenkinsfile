@Library('jenkins_rust') _
rustPipeline(
    enableBenchmarks: true,
    osList: ['linux', 'win', 'osx', 'freebsd', 'web'],
    rustVersion: 'stable',
    repo: 'https://github.com/AnneKitsune/minigene.git',
    branch: 'main',
    buildArgs: '--release',
    artifactPatterns: ['target/release/*', 'target/criterion/**/*']
)

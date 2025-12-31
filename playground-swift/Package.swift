// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "playground-swift",
    platforms: [.macOS(.v10_15)],
    products: [
        .library(name: "DataStructure", targets: ["DataStructure"]),
        .library(name: "n206", targets: ["n206"]),
        .executable(name: "Benchmark", targets: ["Benchmark"])
    ],
    targets: [
        .target(name: "DataStructure"),
        .target(name: "n206", dependencies: ["DataStructure"]),
        .executableTarget(name: "Benchmark", dependencies: ["n206", "DataStructure"]),
        .testTarget(name: "n206Tests", dependencies: ["n206", "DataStructure"])
    ]
)

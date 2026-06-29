// make-icon.swift — render the app icon: the same glassy blue marble the menu bar shows.
//
// Writes a full .iconset (every size macOS wants) that `iconutil` turns into AppIcon.icns.
// One source of truth for the marble's look lives in the Rust marble_icon(); this mirrors its
// palette so Finder and the menu bar agree. Regenerate with packaging/make-icns.sh.
//
// Usage: make-icon <output.iconset-dir>

import CoreGraphics
import Foundation
import ImageIO
import UniformTypeIdentifiers

func renderMarble(_ size: Int) -> CGImage? {
    let s = CGFloat(size)
    let space = CGColorSpaceCreateDeviceRGB()
    guard
        let ctx = CGContext(
            data: nil, width: size, height: size, bitsPerComponent: 8, bytesPerRow: 0,
            space: space, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)
    else { return nil }

    ctx.clear(CGRect(x: 0, y: 0, width: s, height: s))
    let center = CGPoint(x: s / 2, y: s / 2)
    let radius = s / 2 * 0.96
    let circle = CGRect(
        x: center.x - radius, y: center.y - radius, width: radius * 2, height: radius * 2)

    ctx.saveGState()
    ctx.addEllipse(in: circle)
    ctx.clip()

    // Base glass: bright core (120,185,255) → deep rim (18,64,150).
    let base = CGGradient(
        colorsSpace: space,
        colors: [
            CGColor(red: 120 / 255, green: 185 / 255, blue: 255 / 255, alpha: 1),
            CGColor(red: 18 / 255, green: 64 / 255, blue: 150 / 255, alpha: 1),
        ] as CFArray, locations: [0, 1])!
    ctx.drawRadialGradient(
        base, startCenter: center, startRadius: 0, endCenter: center, endRadius: radius,
        options: [])

    // Specular highlight, up and to the left (CG y points up, so +y is "up").
    let hc = CGPoint(x: center.x - radius * 0.35, y: center.y + radius * 0.35)
    let spec = CGGradient(
        colorsSpace: space,
        colors: [
            CGColor(red: 1, green: 1, blue: 1, alpha: 0.9),
            CGColor(red: 1, green: 1, blue: 1, alpha: 0),
        ] as CFArray, locations: [0, 1])!
    ctx.drawRadialGradient(
        spec, startCenter: hc, startRadius: 0, endCenter: hc, endRadius: radius * 0.6, options: [])
    ctx.restoreGState()

    // A soft darker rim for definition.
    ctx.setStrokeColor(CGColor(red: 10 / 255, green: 30 / 255, blue: 80 / 255, alpha: 0.5))
    ctx.setLineWidth(max(1, s * 0.012))
    ctx.addEllipse(in: circle.insetBy(dx: s * 0.01, dy: s * 0.01))
    ctx.strokePath()

    return ctx.makeImage()
}

func writePNG(_ image: CGImage, to path: String) -> Bool {
    let url = URL(fileURLWithPath: path) as CFURL
    guard
        let dest = CGImageDestinationCreateWithURL(url, UTType.png.identifier as CFString, 1, nil)
    else { return false }
    CGImageDestinationAddImage(dest, image, nil)
    return CGImageDestinationFinalize(dest)
}

let args = CommandLine.arguments
guard args.count >= 2 else {
    FileHandle.standardError.write(Data("usage: make-icon <output.iconset>\n".utf8))
    exit(2)
}
let outDir = args[1]
try? FileManager.default.createDirectory(
    atPath: outDir, withIntermediateDirectories: true)

let specs: [(String, Int)] = [
    ("icon_16x16", 16), ("icon_16x16@2x", 32),
    ("icon_32x32", 32), ("icon_32x32@2x", 64),
    ("icon_128x128", 128), ("icon_128x128@2x", 256),
    ("icon_256x256", 256), ("icon_256x256@2x", 512),
    ("icon_512x512", 512), ("icon_512x512@2x", 1024),
]
for (name, px) in specs {
    guard let img = renderMarble(px), writePNG(img, to: "\(outDir)/\(name).png") else {
        FileHandle.standardError.write(Data("make-icon: failed at \(name)\n".utf8))
        exit(3)
    }
}

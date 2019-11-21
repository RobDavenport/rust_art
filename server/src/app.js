async function run() {
    const { PixelMap, GenerationMethod, ColorDepth } = await import("./pkg/client")
    const { memory } = await import("./pkg/client_bg")
    
    const image = PixelMap.new(GenerationMethod.Random, ColorDepth.Bit18)

    const width = image.width()
    const height = image.height()

    const canvas = document.getElementById("canvas")
    canvas.height = height
    canvas.width = width

    /** @type {CanvasRenderingContext2D} */
    const ctx = canvas.getContext('2d')
    const pixelsPtr = image.pixels();
    const pixels = new Uint8Array(memory.buffer, pixelsPtr, width * height * 4)

    console.log("draw pixels")

    let id = ctx.getImageData(0, 0, width, height);
    id.data.set(pixels);
    ctx.putImageData(id, 0, 0);

    console.log("done")
}

run();
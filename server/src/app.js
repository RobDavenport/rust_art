async function run() {
    const { PixelMap } = await import("./pkg/client")
    const { memory } = await import("./pkg/client_bg")

    const image = PixelMap.new()
    const width = image.width()
    const height = image.height()

    const canvas = document.getElementById("canvas")
    canvas.height = height
    canvas.width = width

    /** @type {CanvasRenderingContext2D} */
    const ctx = canvas.getContext('2d')
    const pixelsPtr = image.pixels();
    const pixels = new Uint8Array(memory.buffer, pixelsPtr, width * height * 3)

    console.log("draw pixels")

    let id = ctx.getImageData(0, 0, width, height);
    let d = id.data;

    for (let i = 0; i < height * width; ++i) {
        let renderPixel = i * 3;
        let canvasPixel = i * 4;
        d[canvasPixel] = pixels[renderPixel]; //r
        d[canvasPixel + 1] = pixels[renderPixel + 1]; //g
        d[canvasPixel + 2] = pixels[renderPixel + 2]; //b
        d[canvasPixel + 3] = 255; //a
    }

    ctx.putImageData(id, 0, 0);

    console.log("done")
}

run();
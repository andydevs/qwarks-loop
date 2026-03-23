import { RAFDemoHandler } from "wasm-raf-handler-demo"

try {
    // Get outputs
    let framecount = document.querySelector('#raf-framecount')
    let timestamp = document.querySelector('#raf-timestamp')
    let delta = document.querySelector('#raf-delta')

    // Create new handler
    let handler = RAFDemoHandler.new(framecount, timestamp, delta)

    // Attach buttons
    document
        .querySelector('#ctrl-start')
        .addEventListener('click', (event) => {
            event.stopPropagation()
            handler.start()
        })
    document
        .querySelector('#ctrl-stop')
        .addEventListener('click', (event) => {
            event.stopPropagation()
            handler.stop()
        })
}
catch (error) {
    console.error(error)
}
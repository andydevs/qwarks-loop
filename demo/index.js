import { RAFDemoHandler } from 'qwarks-loop-demo'

try {
    // Create new handler
    let handler = RAFDemoHandler.new(
        document.querySelector('#raf-framecount'),
        document.querySelector('#raf-timestamp'),
        document.querySelector('#raf-delta'),
        document.querySelector('#raf-fps')
    )

    // Attach buttons
    document.querySelector('#ctrl-start').addEventListener('click', (event) => {
        event.stopPropagation()
        handler.start()
    })
    document.querySelector('#ctrl-stop').addEventListener('click', (event) => {
        event.stopPropagation()
        handler.stop()
    })
} catch (error) {
    console.error(error)
}

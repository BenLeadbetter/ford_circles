import init, {get_interface} from "./pkg/ford_circles.js";

let canvas = document.createElement("canvas");
canvas.tabIndex = 0;
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
canvas.id = "circles_canvas"
document.body.append(canvas);

init().then(() => {
    const render_interface = get_interface("circles_canvas");
    
    canvas.addEventListener('keyup', (event) => {
        if (event.code == 'Escape') {
            render_interface.enqueue_quit_action();
        }
    });

    canvas.addEventListener("wheel", (event) => {
        render_interface.enqueue_wheel_action(event.deltaY);
    });

    window.onresize = () => {
        if (
            window.innerWidth !== undefined && window.innerHeight !== undefined
            && window.innerWidth > 0  && window.innerHeight > 0) {
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
            render_interface.enqueue_resize_action(window.innerWidth, window.innerHeight);    
        }
    };

    window.onmousemove = (value) => {
        render_interface.enqueue_cursor_moved_action(value.x, value.y);
    }

    const renderFrame = (_now) => {
        const feedback = render_interface.render();
        if (feedback) {
            window.requestAnimationFrame(renderFrame);
        }
    };

    window.requestAnimationFrame(renderFrame);
})
.catch(console.error);
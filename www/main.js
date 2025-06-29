import init, { CounterGame } from "./pkg/counter_game.js";

let game;

async function main() {
    await init();
    game = new CounterGame("canvas");
    game.draw();

    document.getElementById("incrementBtn").addEventListener("click", () => {
        game.increment();
    });

    window.addEventListener("message", (event) => {
        try {
            const data =
                typeof event.data === "string" ? JSON.parse(event.data) : event.data;

            if (data?.type === "decrement") {
                if (typeof game?.decrement === "function") {
                    game.decrement();
                }
            }
        } catch(error) {
            console.warn("Invalid message format received:", event.data);
        }
    })
}

main();
import init, { CounterGame } from "./pkg/counter_game.js";

let game;

async function main() {
    await init();
    game = new CounterGame("canvas");
    game.draw();

    document.getElementById("incrementBtn").addEventListener("click", () => {
        game.increment();
    });
}

main();
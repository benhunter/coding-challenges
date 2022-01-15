export function decodedValue(colors: string[]): number {
    let colorMap = new Map<string, number>([
            ['black', 0],
            ['brown', 1],
            ['red', 2],
            ['orange', 3],
            ['yellow', 4],
            ['green', 5],
            ['blue', 6],
            ['violet', 7],
            ['grey', 8],
            ['white', 9]
        ]
    )

    if (!colorMap.has(colors[0])) throw new Error('Invalid color');
    if (!colorMap.has(colors[1])) throw new Error('Invalid color');

    let first = colorMap.get(colors[0]) ?? 0;
    let second = colorMap.get(colors[1]) ?? 0;

    return first * 10 + second;
}

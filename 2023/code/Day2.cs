using System.Text.RegularExpressions;

namespace _2023;

public partial class Day2 : IDaySolver {

    public (object, object) Solve(string input) {

        var lines = input.Split("\n");
        var games = new Dictionary<int, List<Pull>>();
        foreach (var line in lines) {
            var gameId = int.Parse(GameIdRegex().Match(line).Groups[1].Value);
            var pullSplit = line.Split(';');
            foreach (var pull in pullSplit) {
                _ = int.TryParse(RedRegex().Match(pull).Groups[1].Value, out int red);
                _ = int.TryParse(GreenRegex().Match(pull).Groups[1].Value, out int green);
                _ = int.TryParse(BlueRegex().Match(pull).Groups[1].Value, out int blue);

                if (!games.TryGetValue(gameId, out List<Pull>? value)) {
                    value = ([]);
                    games[gameId] = value;
                }

                value.Add(new Pull { Red = red, Green = green, Blue = blue });
            }
        }

        var p1Reds = 12;
        var p1Greens = 13;
        var p1Blues = 14;
        var possibleGames = games.Where(g => 
                !g.Value.Any(p => p.Red > p1Reds || p.Green > p1Greens || p.Blue > p1Blues)
            ).ToList();
        var p1Answer = possibleGames.Sum(g => g.Key);

        var p2Answer = 0;
        foreach (var game in games) {
            var pulls = game.Value;
            var gameMinReds = pulls.Max(p => p.Red);
            var gameMinGreens = pulls.Max(p => p.Green);
            var gameMinBlues = pulls.Max(p => p.Blue);
            p2Answer += gameMinReds * gameMinGreens * gameMinBlues;
        }

        return (p1Answer, p2Answer);
    }

    [GeneratedRegex(@"Game (\d+)")]
    private static partial Regex GameIdRegex();
    [GeneratedRegex(@"(\d+) red")]
    private static partial Regex RedRegex();
    [GeneratedRegex(@"(\d+) blue")]
    private static partial Regex BlueRegex();
    [GeneratedRegex(@"(\d+) green")]
    private static partial Regex GreenRegex();
}

internal class Pull {
    public int Red { get; set; } = 0;
    public int Green { get; set; } = 0;
    public int Blue { get; set; } = 0;
    public int Power => Red * Green * Blue;
}
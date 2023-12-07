namespace _2023;

public class Day4 : IDaySolver {

    public (object, object) Solve(string input) {
        var lines = input.Split('\n', StringSplitOptions.TrimEntries);

        var p1points = 0;
        var p2cards = 0;
        var numCards = new Dictionary<int, int>();
        for (int i = 1; i <= 190; i++) {
            numCards.Add(i, 1);
        }

        foreach (var line in lines) {
            var ints = Common.GetIntsFromString(line).ToList();
            var card = ints[0];
            var winning = ints[1..11];
            var having = ints[11..];
            var hits = having.Sum(x => winning.Contains(x) ? 1 : 0);

            var points = (int)Math.Pow(2, hits - 1);
            p1points += points;

            p2cards += numCards[card];
            for (int i = 1; i <= hits; i++) {
                numCards[card + i] += numCards[card];
            }
        }

        return (p1points, p2cards);
    }
}

using System.Text.RegularExpressions;

namespace _2023;

public partial class Day3 : IDaySolver {

    public (object, object) Solve(string input) {
        var lines = input.Split('\n', StringSplitOptions.TrimEntries);
        var partNumbers = new List<int>();
        var gearRatios = new List<int>();

        for (var row = 0; row < lines.Length; row++) {
            for (var col = 0; col < lines[row].Length; col++) {
                if (IsSymbol(lines[row][col])) {
                    
                    var digitLocations = new List<(int, int)>();
                    for (var r = -1; r < 2; r++) {
                        for (var c = -1; c < 2; c++) {
                            if (char.IsDigit(lines[row+r][col+c])) {
                                digitLocations.Add((row+r, col+c));
                            }
                        }
                    }

                    var set = new HashSet<int>();
                    foreach (var dloc in digitLocations) {
                        var left = dloc.Item2;
                        while (true) {
                            if (left == 0) break;
                            if (!char.IsDigit(lines[dloc.Item1][left-1])) break;
                            left--;
                        }
                        var right = dloc.Item2;
                        while (true) {
                            if (right == 139) break;
                            if (!char.IsDigit(lines[dloc.Item1][right+1])) break;
                            right++;
                        }
                        set.Add(int.Parse(lines[dloc.Item1].Substring(left, right - left + 1)));
                    }
                    partNumbers.AddRange(set);

                    // gear check
                    if (lines[row][col] == '*' && set.Count == 2) {
                        gearRatios.Add(set.First() * set.Last());
                    }
                }
            }
        }

        return (partNumbers.Sum(), gearRatios.Sum());
    }

    private bool IsSymbol(char ch) {
        return ch != '.' && !char.IsDigit(ch);
    }
}

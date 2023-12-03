using System.Text.RegularExpressions;

namespace _2023;

public partial class Day1 : IDaySolver {

    public (object, object) Solve(string input) {

        var lines = input.Split("\n");

        var p1sum = 0;
        var digitRegex = SingleDigitRegex();
        foreach (var line in lines) {
            var matches = digitRegex.Matches(line);
            if (matches.Count == 0) continue;
            var number = matches[0].Value + matches[^1].Value;
            p1sum += int.Parse(number);
        }

        var p2sum = 0;
        foreach (var line in lines) {
            var firstDigit = string.Empty;
            for (var i = 0; i < line.Length; i++) {
                if (char.IsDigit(line[i])) {
                    firstDigit = line[i].ToString();
                    break;
                } else if (line.Length - i >= 3 && line.Substring(i, 3) == "one") {
                    firstDigit = "1";
                    break;
                } else if (line.Length - i >= 3 && line.Substring(i, 3) == "two") {
                    firstDigit = "2";
                    break;
                } else if (line.Length - i >= 5 && line.Substring(i, 5) == "three") {
                    firstDigit = "3";
                    break;
                } else if (line.Length - i >= 4 && line.Substring(i, 4) == "four") {
                    firstDigit = "4";
                    break;
                } else if (line.Length - i >= 4 && line.Substring(i, 4) == "five") {
                    firstDigit = "5";
                    break;
                } else if (line.Length - i >= 3 && line.Substring(i, 3) == "six") {
                    firstDigit = "6";
                    break;
                } else if (line.Length - i >= 5 && line.Substring(i, 5) == "seven") {
                    firstDigit = "7";
                    break;
                } else if (line.Length - i >= 5 && line.Substring(i, 5) == "eight") {
                    firstDigit = "8";
                    break;
                } else if (line.Length - i >= 4 && line.Substring(i, 4) == "nine") {
                    firstDigit = "9";
                    break;
                }
            }
            var lastDigit = string.Empty;
            for (var k = line.Length - 1; k >= 0; k--) {
                if (char.IsDigit(line[k])) {
                    lastDigit = line[k].ToString();
                    break;
                } else if (k >= 2 && line.Substring(k - 2, 3) == "one") {
                    lastDigit = "1";
                    break;
                } else if (k >= 2 && line.Substring(k - 2, 3) == "two") {
                    lastDigit = "2";
                    break;
                } else if (k >= 4 && line.Substring(k - 4, 5) == "three") {
                    lastDigit = "3";
                    break;
                } else if (k >= 3 && line.Substring(k - 3, 4) == "four") {
                    lastDigit = "4";
                    break;
                } else if (k >= 3 && line.Substring(k - 3, 4) == "five") {
                    lastDigit = "5";
                    break;
                } else if (k >= 2 && line.Substring(k - 2, 3) == "six") {
                    lastDigit = "6";
                    break;
                } else if (k >= 4 && line.Substring(k - 4, 5) == "seven") {
                    lastDigit = "7";
                    break;
                } else if (k >= 4 && line.Substring(k - 4, 5) == "eight") {
                    lastDigit = "8";
                    break;
                } else if (k >= 3 && line.Substring(k - 3, 4) == "nine") {
                    lastDigit = "9";
                    break;
                }
            }
            // Console.WriteLine($"{line} -> {firstDigit}{lastDigit}");
            p2sum += int.Parse(firstDigit + lastDigit);
        }

        return (p1sum, p2sum);
    }

    [GeneratedRegex(@"\d")]
    private static partial Regex SingleDigitRegex();
}
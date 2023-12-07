using System.Linq;
using System.Text.RegularExpressions;

namespace _2023;

public static partial class Common {
    public static IEnumerable<int> GetIntsFromString(string str) {
        var matches = IntsInStringRegex().Matches(str);
        return matches.Select(m => int.Parse(m.Value)).AsEnumerable();
    }

    [GeneratedRegex(@"(\d+)")]
    private static partial Regex IntsInStringRegex();
}
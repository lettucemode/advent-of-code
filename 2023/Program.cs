using System.Diagnostics;
using _2023;

internal class Program {

    private static void Main(string[] args) {
        var totalMsTaken = 0L;
        for (var i = 1; i <= 25; i++) {
            var t = Type.GetType($"_2023.Day{i}");
            if (t == null) continue;
            var instance = (IDaySolver)Activator.CreateInstance(t)!;
            totalMsTaken += SolveDay(i, instance.Solve);
        }
        Console.WriteLine();
        Console.WriteLine($"Total time taken: {totalMsTaken / 1000:N0}s {totalMsTaken % 1000:N0}ms");
    }

    private static long SolveDay(int day, Solver solver) {
        var input = File.ReadAllText($"inputs/day{day}.txt");
        var stopwatch = Stopwatch.StartNew();   
        var (p1, p2) = solver(input);
        stopwatch.Stop();
        var formattedTime = stopwatch.ElapsedMilliseconds.ToString("N0");
        Console.WriteLine();
        Console.WriteLine($"Day {day}: ({p1} {p2})");
        Console.WriteLine($"  Time: {formattedTime}ms");
        return stopwatch.ElapsedMilliseconds;
    }

    private delegate (object, object) Solver(string input);
}
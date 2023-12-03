using System.Diagnostics;
using _2023;

internal class Program
{
    private static void Main(string[] args)
    {
        SolveDay(1, new Day1().Solve);
        SolveDay(2, new Day2().Solve);
    }

    private static void SolveDay(int day, Solver solver)
    {
        var input = File.ReadAllText($"inputs/Day{day}.txt");
        var stopwatch = Stopwatch.StartNew();   
        var (p1, p2) = solver(input);
        stopwatch.Stop();
        var formattedTime = stopwatch.ElapsedMilliseconds.ToString("N0");
        Console.WriteLine();
        Console.WriteLine($"Day {day}: ({p1} {p2})");
        Console.WriteLine($"  Time: {formattedTime}ms");
    }

    private delegate (object, object) Solver(string input);
}
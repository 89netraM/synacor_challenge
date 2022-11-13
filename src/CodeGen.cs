using System;
using System.Text;

public static partial class Assembler
{
	private const string CodeHeader = """
		using System;
		using System.Collections.Generic;
		using System.IO;

		public static class Program
		{
			public static void Main()
			{
				byte[] bytes = File.ReadAllBytes("./memory.bin");
				ushort[] memory = new ushort[bytes.Length / 2];
				Buffer.BlockCopy(bytes, 0, memory, 0, bytes.Length);

				Stack<ushort> stack = new Stack<ushort>();

				ushort a = 0, b = 0, c = 0, d = 0, e = 0, f = 0, g = 0, h = 0;

				ushort ip = 0;

				unchecked
				{
				START:
					switch (ip)
					{
		""";
	private const string CodeFooter = """
					}
				}
			}
		}
		""";

	private static string BuildCode(ushort[] assembly)
	{
		var code = new StringBuilder(CodeHeader);

		// The code is self-editing
		assembly[937] = 21;
		assembly[938] = 7;

		ReadOnlySpan<ushort> span = assembly;
		for (ushort instruction = 0; span.Length > 0 && instruction < 6068;)
		{
			if (instruction > 0)
			{
				code.AppendLine($"goto case {instruction};");
			}
			code.AppendLine($"case {instruction}:");
			code.AppendLine($"ip = {instruction};");

			var width = BuildOp(span[0], span[1..], code);
			span = span[width..];
			instruction += width;
		}
		code.AppendLine($"break;");

		code.Append(CodeFooter);
		return code.ToString();
	}

	private static ushort BuildOp(ushort op, ReadOnlySpan<ushort> span, StringBuilder codeBuilder)
	{
		var (code, size) = op switch
		{
			0 => (BuildHalt(), 1),
			1 => (BuildSet(span[0], span[1]), 3),
			2 => (BuildPush(span[0]), 2),
			3 => (BuildPop(span[0]), 2),
			4 => (BuildEq(span[0], span[1], span[2]), 4),
			5 => (BuildGt(span[0], span[1], span[2]), 4),
			6 => (BuildJmp(span[0]), 2),
			7 => (BuildJt(span[0], span[1]), 3),
			8 => (BuildJf(span[0], span[1]), 3),
			9 => (BuildAdd(span[0], span[1], span[2]), 4),
			10 => (BuildMult(span[0], span[1], span[2]), 4),
			11 => (BuildMod(span[0], span[1], span[2]), 4),
			12 => (BuildAnd(span[0], span[1], span[2]), 4),
			13 => (BuildOr(span[0], span[1], span[2]), 4),
			14 => (BuildNot(span[0], span[1]), 3),
			15 => (BuildRmem(span[0], span[1]), 3),
			16 => (BuildWmem(span[0], span[1]), 3),
			17 => (BuildCall(span[0]), 2),
			18 => (BuildRet(), 1),
			19 => (BuildOut(span[0]), 2),
			20 => (BuildIn(span[0]), 2),
			_ => (null, 1),
		};
		if (code is not null)
		{
			codeBuilder.AppendLine(code);
		}
		return (ushort)size;
	}

	private static string BuildHalt() =>
		"return;";

	private static string BuildSet(ushort register, ushort value) =>
		$"{ArgToExpr(register)} = {ArgToExpr(value)};";

	private static string BuildPush(ushort value) =>
		$"stack.Push({ArgToExpr(value)});";

	private static string BuildPop(ushort register) =>
		$"{ArgToExpr(register)} = stack.Pop();";

	private static string BuildEq(ushort register, ushort a, ushort b) =>
		$"{ArgToExpr(register)} = (ushort)({ArgToExpr(a)} == {ArgToExpr(b)} ? 1 : 0);";

	private static string BuildGt(ushort register, ushort a, ushort b) =>
		$"{ArgToExpr(register)} = (ushort)({ArgToExpr(a)} > {ArgToExpr(b)} ? 1 : 0);";

	private static string BuildJmp(ushort target) =>
		$"""
			ip = {ArgToExpr(target)};
			goto START;
			""";

	private static string BuildJt(ushort cond, ushort target) =>
		$$"""
			if ({{ArgToExpr(cond)}} != 0)
			{
				ip = {{ArgToExpr(target)}};
				goto START;
			}
			""";

	private static string BuildJf(ushort cond, ushort target) =>
		$$"""
			if ({{ArgToExpr(cond)}} == 0)
			{
				ip = {{ArgToExpr(target)}};
				goto START;
			}
			""";

	private static string BuildAdd(ushort register, ushort a, ushort b) =>
		$"{ArgToExpr(register)} = (ushort)((({ArgToExpr(a)} + {ArgToExpr(b)}) % 32768 + 32768) % 32768);";

	private static string BuildMult(ushort register, ushort a, ushort b) =>
		$"{ArgToExpr(register)} = (ushort)((({ArgToExpr(a)} * {ArgToExpr(b)}) % 32768 + 32768) % 32768);";

	private static string BuildMod(ushort register, ushort a, ushort b) =>
		$"{ArgToExpr(register)} = (ushort)({ArgToExpr(a)} % {ArgToExpr(b)});";

	private static string BuildAnd(ushort register, ushort a, ushort b) =>
		$"{ArgToExpr(register)} = (ushort)({ArgToExpr(a)} & {ArgToExpr(b)});";

	private static string BuildOr(ushort register, ushort a, ushort b) =>
		$"{ArgToExpr(register)} = (ushort)({ArgToExpr(a)} | {ArgToExpr(b)});";

	private static string BuildNot(ushort register, ushort value) =>
		$"{ArgToExpr(register)} = (ushort)(0x7FFF ^ {ArgToExpr(value)});";

	private static string BuildRmem(ushort register, ushort address) =>
		$"{ArgToExpr(register)} = memory[(int){ArgToExpr(address)}];";

	private static string BuildWmem(ushort address, ushort value) =>
		$"memory[(int){ArgToExpr(address)}] = {ArgToExpr(value)};";

	private static string BuildCall(ushort target) =>
		$$"""
			stack.Push((ushort)(ip + 2));
			ip = {{ArgToExpr(target)}};
			goto START;
			""";

	private static string BuildRet() =>
		$$"""
			if (stack.TryPop(out ip))
			{
				goto START;
			}
			else
			{
				return;
			}
			""";

	private static string BuildOut(ushort output) =>
		$"Console.Write((char){ArgToExpr(output)});";

	private static string BuildIn(ushort register) =>
		$$"""
			{
				int i;
				while ((i = Console.Read()) == '\r');
				{{ArgToExpr(register)}} = (ushort)i;
			}
			""";

	private static string ArgToExpr(ushort arg)
	{
		if (arg > 32767)
		{
			return ((char)(arg - 32768 + 'a')).ToString();
		}
		else
		{
			return arg.ToString();
		}
	}
}

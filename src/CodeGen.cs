using System;
using System.Text;

public static partial class Assembler
{
	private const string CodeHeader = """
		using System;

		public static class Program
		{
			public static void Main()
			{
				ushort ip = 0;
			START:
				switch (ip)
				{
		""";
	private const string CodeFooter = """
				}
			}
		}
		""";

	private static string BuildCode(ushort[] assembly)
	{
		var code = new StringBuilder(CodeHeader);

		ReadOnlySpan<ushort> span = assembly;
		for (ushort instruction = 0; span.Length > 0;)
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

	private static ushort BuildOp(ushort op, ReadOnlySpan<ushort> span, StringBuilder code)
	{
		switch (op)
		{
			case 0:
				BuildHalt(code);
				return 1;
			case 19:
				BuildOut(span[0], code);
				return 2;
			default:
				return 1;
		}
	}

	private static void BuildHalt(StringBuilder code)
	{
		code.AppendLine("return;");
	}

	private static void BuildOut(ushort output, StringBuilder code)
	{
		code.AppendLine($"Console.Write((char){output});");
	}
}

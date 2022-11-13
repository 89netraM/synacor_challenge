using System;
using System.IO;
using System.Reflection;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;

public static partial class Assembler
{
	public static void Compile(ushort[] assembly, string outputPath)
	{
		var code = BuildCode(assembly);
		MakeFunction(code, outputPath);
	}

	private static void MakeFunction(string code, string outputPath)
	{
		var compilation = CSharpCompilation.Create(
			Path.GetRandomFileName(),
			new[] { CSharpSyntaxTree.ParseText(code) },
			new[]
			{
				MetadataReference.CreateFromFile(Assembly.Load(new AssemblyName("System.Private.CoreLib")).Location),
				MetadataReference.CreateFromFile(Assembly.Load(new AssemblyName("System.Runtime")).Location),
				MetadataReference.CreateFromFile(Assembly.Load(new AssemblyName("System.Console")).Location),
				MetadataReference.CreateFromFile(Assembly.Load(new AssemblyName("System.Collections")).Location),
			},
			new CSharpCompilationOptions(
#if DEBUG
				optimizationLevel: OptimizationLevel.Debug,
#else
				optimizationLevel: OptimizationLevel.Release,
#endif
				outputKind: OutputKind.ConsoleApplication
			));

		var emitResult = compilation.Emit(outputPath);

		if (!emitResult.Success)
		{
			Console.Error.WriteLine("Assembler compilation errors:");
			foreach (var error in emitResult.Diagnostics)
			{
				if (error.Severity == DiagnosticSeverity.Error)
				{
					Console.Error.WriteLine($"\t{error.GetMessage()}");
				}
			}
		}
	}
}

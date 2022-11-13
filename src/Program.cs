using System;
using System.IO;

var bytes = File.ReadAllBytes("./challenge.bin");
var assembly = new ushort[bytes.Length / 2];
Buffer.BlockCopy(bytes, 0, assembly, 0, bytes.Length);

Assembler.Compile(assembly, "./program.exe");

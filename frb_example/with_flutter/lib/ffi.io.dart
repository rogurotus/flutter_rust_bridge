import 'dart:io';
// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/utils.io.dart';
import 'bridge_generated.io.dart';

const base = 'flutter_rust_bridge_example';
final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
late final dylib = loadDylib(path);
late final api = FlutterRustBridgeExampleImpl(dylib);

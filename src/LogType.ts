export enum LogOutputValueType {
  Undefined = "undefined",
  Null = "null",
  Boolean = "boolean",
  Number = "number",
  String = "string",
  Array = "array",
  Object = "object",
  BigInt = "bigint",
  Date = "date",
}




export type LogOutput = {
  type: LogOutputValueType.Undefined;
  value: string;
} | {
  type: LogOutputValueType.Null;
  value: null;
} | {
  type: LogOutputValueType.Boolean;
  value: boolean;
} | {
  type: LogOutputValueType.Number;
  value: number;
} | {
  type: LogOutputValueType.String;
  value: string;
} | {
  type: LogOutputValueType.Array;
  value: LogOutput[];
} | {
  type: LogOutputValueType.Object;
  value: Record<string, LogOutput>;
} | {
  type: LogOutputValueType.BigInt;
  value: string;
} | {
  type: LogOutputValueType.Date;
  value: string;
} | any
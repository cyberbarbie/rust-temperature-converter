# https://nim-lang.org - Compile & Run:  nim r main.nim
import strutils

proc main() =
  echo "Please enter kelvin"
  let kelvin = try: stdin.readLine.parseInt except: quit "Failed to read line"

  let celsius = kelvin - 273
  let fahrenheit = int(float(celsius) * 1.8 + 32)
  let newton = int(float(celsius) * 0.33)

  echo "The temperature converted from kelvin to celsius is ", celsius
  echo "The temperature converted from celsius to fahrenheit is ", fahrenheit
  echo "The Newton temperature is ", newton

main()

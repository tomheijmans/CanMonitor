#include <SD.h>

File demoFile;
const int SPI_CS_SD     = 9;

void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);

  if (!SD.begin(SPI_CS_SD)) {
      Serial.println("SD initialization failed!");
      while (1);
  }

  demoFile = SD.open("Demo.CSV");
}

void loop() {
  String line = readLine();
  if(line == ""){
    demoFile.close();
    demoFile = SD.open("Demo.CSV");
  }else{
    Serial.println(line);
  }
  //delay(10);
}

String readLine()
{
  String received = "";
  char caracter;
  while (demoFile.available())
  {
    caracter = demoFile.read();
    if (caracter == '\n')
    {
      return String(received);
    }
    else
    {
      received += caracter;
    }
  }
  return "";
}

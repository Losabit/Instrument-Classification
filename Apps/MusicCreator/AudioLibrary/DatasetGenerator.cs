using NAudio.Wave;
using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;

namespace AudioLibrary
{
    //Quelques problèmes quand on augmente le MinMix
    public class GeneratorMix
    {
        public List<string> Paths = new List<string>();
        public string OutDirectory = string.Empty;
        public string OutFileName = string.Empty;
        public int MinMix;
        public int MaxMix; 

        public GeneratorMix(List<string> paths)
        {
            MinMix = 1;
            MaxMix = MinMix + 1;
            Paths = paths;
        }

        public GeneratorMix(List<string> paths, string outDirectory, string outFileName)
        {
            MinMix = 1;
            MaxMix = MinMix + 1;
            OutDirectory = outDirectory;
            OutFileName = outFileName;
            Paths = paths;
        }

        public void GenerateMix(int numberOfRecord)
        {
            if (MinMix > MaxMix || MinMix <= 0 || MaxMix > Paths.Count)
                throw new Exception("Mauvaises valeurs pour minMix et maxMix");

            List<FileInfo> files = new List<FileInfo>();
            for (int i = 0; i < Paths.Count; i++)
                files.Add(new FileInfo(Paths[i]));

            Random rand = new Random();
            List<int> usedIndices = new List<int>();
            for (int i = 0; i < numberOfRecord; i++)
            {
                var mixer = new WaveMixerStream32 { AutoStop = true };
                int stopMix = rand.Next(MinMix, MaxMix + 1);

                for (int j = 0; j < stopMix; j++)
                {
                    int indiceFile = 0;
                    do
                    {
                       indiceFile = rand.Next(0, Paths.Count);
                    } while (usedIndices.Contains(indiceFile));

                    usedIndices.Add(indiceFile);
                    mixer.AddInputStream(new WaveChannel32(new WaveFileReader(Paths[indiceFile])));
                }
                usedIndices.Clear();
                WaveFileWriter.CreateWaveFile(GetPath(i + 1), new Wave32To16Stream(mixer));
            }
        }

        public string GetFileName(int indice)
        {
            return OutFileName.Insert(OutFileName.LastIndexOf("."), indice.ToString());
        }

        public string GetPath(int indice)
        {
            if (string.IsNullOrEmpty(OutDirectory))
                return string.Empty;
            return OutDirectory + "\\" + GetFileName(indice);
        }
    }

    public class DatasetGenerator
    {
        public class Labels
        {
            public string Path { get; set; }
            public string Label { get; set; }
        }

        public enum Instrument
        {
            Piano,
            Guitare,
            Saxophone
        }

        public List<string> Paths;
        public Instrument instrument;

        public DatasetGenerator(Instrument instrument, List<string> paths)
        {
            this.instrument = instrument;
            Paths = paths;
        }

        public bool Verify()
        {
            return false;
        }

        public void ToDirectory(string directoryPath, string fileName)
        {
            for(int i = 0; i < Paths.Count; i++)
            {
                File.Copy(Paths[i], directoryPath + Path.DirectorySeparatorChar + fileName.Replace(".wav", "") + (i + 1) + ".wav");
            }
        }

        public void ToCsv(string outpath, bool exist)
        {
            //using (var reader = new StreamReader(outpath))
            //using (var csv = new CsvReader(reader, CultureInfo.InvariantCulture))
            //{
            //    var records = csv.GetRecords<Labels>();
            //}
        }
    }
}

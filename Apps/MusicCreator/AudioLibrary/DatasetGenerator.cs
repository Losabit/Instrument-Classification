using CsvHelper;
using NAudio.Wave;
using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Numerics;

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
        public class LabelsWav
        {
            public string Valeur { get; set; }
        }

        public class LabelsFFT
        {
            public string Valeur { get; set; }
            public string Frequence { get; set; }
        }

        public enum Instrument
        {
            Piano,
            Guitare,
            Saxophone
        }

        public enum StockageMode
        {
            Directory,
            CSV
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

        public void Create(StockageMode stockageMode, string outpath)
        {
            if (stockageMode == StockageMode.Directory)
                ToDirectory(outpath);
            else if (stockageMode == StockageMode.CSV)
                ToCsv(outpath);
        }

        private void ToDirectory(string outpath)
        {
            for(int i = 0; i < Paths.Count; i++)
            {
                File.Copy(Paths[i], outpath + Path.DirectorySeparatorChar + Enum.GetName(typeof(Instrument), instrument).Replace(".wav", "") + (i + 1) + ".wav");
            }
        }

        private void ToCsv(string outpath)
        {
            for (int i = 0; i < Paths.Count; i++)
            {
                WavFile wav = WavFile.Read(Paths[i]);
                Complex[] complexs = wav.GetComplexData();
                string path = outpath + Path.DirectorySeparatorChar + Enum.GetName(typeof(Instrument), instrument) + (i + 1).ToString() + ".csv";
                FileStream fs = File.Create(path);
                fs.Close();
                using(var csv = new CsvWriter(new StreamWriter(path), CultureInfo.InvariantCulture))
                {
                    csv.WriteHeader(typeof(LabelsWav));
                    csv.NextRecord();
                    for (int j = 0; j < complexs.Length; j++)
                    {
                        csv.WriteField(complexs[j].Real.ToString());
                        csv.NextRecord();
                    }
                }
            }
        }
    }
}

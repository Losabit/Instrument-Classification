using CsvHelper;
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

        private int minMix;
        public int MinMix
        {
            get { return minMix; }
            set
            {
                if (value > maxMix || value < 0)
                    return;
                minMix = value;
            }
        }

        private int maxMix;
        public int MaxMix
        {
            get { return maxMix; }
            set
            {
                if (value < minMix)
                    return;
                maxMix = value;
            }
        }

        public GeneratorMix(List<string> InDirectory)
        {
            MinMix = 1;
            MaxMix = MinMix + 1;
        }

        public GeneratorMix(List<string> InDirectory, string outDirectory, string outFileName)
        {
            MinMix = 1;
            MaxMix = MinMix + 1;
            OutDirectory = outDirectory;
            OutFileName = outFileName;
            Paths = InDirectory;
        }

        public void GenerateMix(int numberOfRecord)
        {
            List<FileInfo> files = new List<FileInfo>();
            for (int i = 0; i < Paths.Count; i++)
                files.Add(new FileInfo(Paths[i]));

            Random rand = new Random();
            for (int i = 0; i < numberOfRecord; i++)
            {
                var mixer = new WaveMixerStream32 { AutoStop = true };
                int stopMix = rand.Next(MinMix, MaxMix + 1);

                for (int j = 0; j < stopMix; j++)
                {
                    int indiceFile = rand.Next(0, Paths.Count);
                    mixer.AddInputStream(new WaveChannel32(new WaveFileReader(Paths[indiceFile])));
                }
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

        public void ToCsv(string outpath, bool exist, bool writeLabel)
        {
            using (var reader = new StreamReader(outpath))
            using (var csv = new CsvReader(reader, CultureInfo.InvariantCulture))
            {
                var records = csv.GetRecords<Labels>();
            }
        }
    }
}

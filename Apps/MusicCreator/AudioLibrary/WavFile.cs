using System;
using System.IO;
using System.Numerics;
using System.Text;


namespace AudioLibrary
{
    public class WavFile
    {
        private int HeaderSize { get { return 28 + BlocSize; } }
        public Int32 DataSize = 0;
        public string TypeBloc = "RIFF";
        public string Format = "WAVE";
        public string FormatBloc = "fmt ";
        public Int32 BlocSize = 16;
        public Int16 AudioFormat = 1;
        public Int16 Canaux = 1;
        public Int32 Frequence;
        public Int16 BitsPerSample = 8; // appeler une fonction lors de sa modification
        public Int16 BytePerBloc { get { return (short)(Canaux * BitsPerSample / 8); } }
        public Int32 BytePerSec { get { return Frequence * BytePerBloc; } }
        public byte[] Data = null;
        public string Path = string.Empty;
        public double Seconds
        {
            get
            {
                if (Data == null)
                    return (double)DataSize / (double)BytePerSec;
                else
                    return ((double)Data.Length) / ((double)BytePerSec);
            }
        }

        public WavFile(Int32 frequence)
        {
            Frequence = frequence;
        }

        public WavFile(string typeBloc, string format, string formatBloc, Int16 audioFormat, Int16 canaux, Int32 frequence, Int16 bitsPerSample)
        {
            TypeBloc = typeBloc;
            Format = format;
            FormatBloc = formatBloc;
            AudioFormat = audioFormat;
            Canaux = canaux;
            Frequence = frequence;
            BitsPerSample = bitsPerSample;
        }

        public double GetSecond(int indice)
        {
            return (double)indice / BytePerSec;
        }

        public int GetIndice(double second)
        {
            return (int)(second * BytePerSec);
        }

        public void Create(string path)
        {
            Create(path, Data);
        }

        public void Create(string path, byte[] data)
        {
            if (data == null)
                throw new Exception("Data can't be null");

            Path = path;
            using (BinaryWriter br = new BinaryWriter(File.Create(path, 1024 * 1024)))
            {
                br.Write(Encoding.ASCII.GetBytes(TypeBloc));
                br.Write(HeaderSize + data.Length - 8);
                br.Write(Encoding.ASCII.GetBytes(Format));
                br.Write(Encoding.ASCII.GetBytes(FormatBloc));
                br.Write(BlocSize);
                br.Write(AudioFormat);
                br.Write(Canaux);
                br.Write(Frequence);
                br.Write(BytePerSec);
                br.Write(BytePerBloc);
                br.Write(BitsPerSample);
                for (int i = 16; i < BlocSize; i += 2)
                    br.Write((Int16)0);
                br.Write(Encoding.ASCII.GetBytes("data"));
                br.Write(data.Length);
                br.Write(data);
            }
        }

        public static WavFile[] ReadDirectory(string path)
        {
            DirectoryInfo d = new DirectoryInfo(path);
            FileInfo[] files = d.GetFiles("*.wav");
            WavFile[] waves = new WavFile[files.Length];
            for (int i = 0; i < waves.Length; i++)
            {
                waves[i] = Read(path + "\\" + files[i].Name);
            }
            return waves;
        }

        public static WavFile Read(string path, bool withData = true)
        {
            if (string.IsNullOrEmpty(path))
                throw new Exception("Bad path");

            byte[] content = File.ReadAllBytes(path);
            if (content.Length < 44)
                Console.WriteLine("Bad Wavfile header");

            string typeBloc = Common.BytesToString(content, 0, 4);
            string format = Common.BytesToString(content, 8, 4);
            string formatBloc = Common.BytesToString(content, 12, 4);
            Int32 blocSize = BitConverter.ToInt32(content, 16);
            Int16 audioFormat = BitConverter.ToInt16(content, 20);
            Int16 canaux = BitConverter.ToInt16(content, 22);
            Int32 frequence = BitConverter.ToInt32(content, 24);
            Int16 bitsPerSample = BitConverter.ToInt16(content, 34);
            WavFile wavFile = new WavFile(typeBloc, format, formatBloc, audioFormat, canaux, frequence, bitsPerSample);
            wavFile.DataSize = BitConverter.ToInt32(content, 40 + (blocSize - 16));
            wavFile.BlocSize = blocSize;
            if (withData)
                wavFile.Data = Common.CutArray<byte>(content, wavFile.HeaderSize, content.Length - wavFile.HeaderSize);
            wavFile.Path = path;
            return wavFile;
        }

        public void ReadData(double start, double end)
        {
            if (string.IsNullOrEmpty(Path))
                return;
            using (FileStream file = File.OpenRead(Path))
            {
                byte[] data = new byte[GetIndice(end) - GetIndice(start)];
                file.Seek(GetIndice(start), SeekOrigin.Begin);
                using (BinaryReader reader = new BinaryReader(file))
                {
                    Data = reader.ReadBytes(data.Length);
                }
              //  file.Read(data, 2, data.Length);
                file.Close();
                
            }
        }

        public WavFile Clone(bool withData = true)
        {
            WavFile clonedWav = new WavFile(TypeBloc, Format, FormatBloc, AudioFormat, Canaux, Frequence, BitsPerSample);
            if (withData)
                clonedWav.Data = Data;
            return clonedWav;
        }

        public Complex[] GetComplexData()
        {
            if (Data == null)
                return null;

            Complex[] complexs = new Complex[Data.Length / (BitsPerSample / 8)];
            for (int i = 0; i < Data.Length; i++)
            {
                if (BitsPerSample == 8)
                    complexs[i] = new Complex(Data[i], 0);
                else if (BitsPerSample == 16)
                {
                    if (i + 1 != Data.Length)
                        complexs[i / 2] = new Complex((ushort)((Data[i + 1] << 8) | (Data[i] & 0xff)), 0);
                    i++;
                }
                else
                    throw new Exception("24 and 32 BitsPerSample not implemented in GetComplex Data");
            }
            return complexs;
        }

        public Complex[] GetDecibelData()
        {
            Complex[] complexs = GetComplexData();
            if (complexs == null)
                return null;

            int maxValue = (int)Math.Pow(2, BitsPerSample) - 1;
            for (int i = 0; i < complexs.Length; i++)
            {
                double value = (complexs[i].Real / maxValue) * 2;
                if (value >= 1)
                    value = value - 2;
                complexs[i] = new Complex(value, 0);
            }
            return complexs;
        }

        public WavFile[] ToMono(bool withData = true)
        {
            if (Data == null)
                throw new Exception("Data of WavFile is null");

            WavFile[] result = new WavFile[Canaux];

            for (int i = 0; i < Canaux; i++)
            {
                result[i] = Clone(false);
                result[i].Canaux = 1;
                result[i].Path = Path;

                if (withData)
                {
                    byte[] data = new byte[(Data.Length / Canaux) + 1];
                    int dataSize = 0;

                    for (int j = i * (BitsPerSample / 8); j < Data.Length; j += BytePerBloc)
                    {
                        for (int k = 0; k < BitsPerSample / 8; k++)
                        {
                            data[dataSize] = Data[j + k];
                            dataSize++;
                        }
                    }
                    result[i].Data = data;
                }
            }
            return result;
        }

        public WavFile[] Cut(double start, double end, int numberOfResult)
        {
            if (end > Seconds)
                throw new Exception("Wav file can't be cut because the cuttime is larger than the file");
            if (Data == null)
                throw new Exception("Data is null");

            WavFile[] waves = new WavFile[numberOfResult];
            double second = (end - start) / numberOfResult;

            for (int i = 0; i < numberOfResult; i++)
            {
                waves[i] = Clone(false);
                waves[i].Data = Common.CutArray(Data, (int)(BytePerSec * second * i + (BytePerSec * start)), (int)(BytePerSec * second));
            }
            return waves;
        }

        public WavFile CutOne(int start, int second)
        {
            WavFile wave = Clone(false);
            wave.Data = Common.CutArray(Data, BytePerSec * start, BytePerSec * second);
            return wave;
        }

        public WavFile WhiteWave(int seconds)
        {
            WavFile wav = Clone(false);
            byte[] data = new byte[seconds * BytePerSec];
            for (int i = 0; i < data.Length; i++)
                data[i] = 0;
            wav.Data = data;
            return wav;
        }

        /// Comment sont dessinés les valeurs négatives sur Audacity :
        ///     (byte) (255 + 123) = 123       Valeur positive 
        ///     (byte) (255 - 123) = 132       Valeur négative
        ///     Centre -> 0 ou 255
        public static double[] ToDecibel(Complex[] complexs, int bitsPerSample)
        {
            int maxValue = (int)Math.Pow(16, bitsPerSample / 4) - 1;
            double[] values = new double[complexs.Length];

            for (int i = 0; i < complexs.Length; i++)
            {
                values[i] = (complexs[i].Real / 255) * 2;
                if (values[i] >= 1)
                    values[i] = 2 - values[i];
            }
            return values;
        }

        public static WavFile Regroupe(params WavFile[] waves)
        {
            if (waves == null || waves.Length < 2)
                throw new Exception("Regroupe can compute one or less file");

            int size = 0;
            for (int i = 0; i < waves.Length - 1; i++)
            {
                if (waves[i].Canaux != waves[i + 1].Canaux || waves[i].BitsPerSample != waves[i + 1].BitsPerSample)
                    throw new Exception("not implemented");
                size += waves[i].Data.Length;
            }
            size += waves[waves.Length - 1].Data.Length;

            WavFile wav = waves[0].Clone(false);
            byte[] data = new byte[size];
            int actualSize = 0;
            for (int i = 0; i < waves.Length; i++)
            {
                for (int j = 0; j < waves[i].Data.Length; j++)
                {
                    data[actualSize] = waves[i].Data[j];
                    actualSize++;
                }
            }
            wav.Data = data;
            return wav;
        }

        public static byte[] DataFusion(WavFile wav, WavFile wav2)
        {
            if (wav.Canaux != wav2.Canaux || wav.BitsPerSample != wav2.BitsPerSample || wav.Frequence != wav2.Frequence)
                throw new Exception("WavFiles can't be merge");

            if (wav.BitsPerSample == 8)
                return DataFusion(wav.Data, wav2.Data);
            else if (wav.BitsPerSample == 16)
                return DataFusion(Common.DataConverter8To16(wav.Data), Common.DataConverter8To16(wav2.Data));
            else
                return null;
        }

        private static byte[] DataFusion(byte[] data, byte[] data2)
        {
            byte[] result;
            if (data2.Length > data.Length)
            {
                result = data2;
                for (int i = 0; i < data.Length; i++)
                    result[i] += data[i];
            }
            else
            {
                result = data;
                for (int i = 0; i < data2.Length; i++)
                    result[i] += data2[i];
            }
            return result;
        }

        private static byte[] DataFusion(ushort[] data, ushort[] data2)
        {
            ushort[] result;
            double presenceData = 0.7;
            double presenceData2 = 1 - presenceData;
            if (data2.Length > data.Length)
            {
                result = new ushort[data2.Length];
                for (int i = 0; i < data2.Length; i++)
                {
                    if (i < data.Length)
                        result[i] = (ushort)(data[i] + data2[i]);
                    else
                        result[i] = (ushort)(data2[i]);
                }
            }
            else
            {
                result = new ushort[data.Length];
                for (int i = 0; i < data.Length; i++)
                {
                    if (i < data2.Length)
                    {
                        result[i] = (ushort)(data[i] + data2[i]);
                    }
                    else
                        result[i] = (ushort)(data[i]);
                }
            }
            return Common.DataConverter16To8(result);
        }

        public override string ToString()
        {
            string result = "--- Header ---\n";
            result += "HeaderSize : " + HeaderSize + "\n";
            result += "TypeBloc : " + TypeBloc + "\n";
            result += "Format : " + Format + "\n";
            result += "FormatBloc : " + FormatBloc + "\n";
            result += "BlocSize : " + BlocSize + "\n";
            result += "AudioFormat : " + AudioFormat + "\n";
            result += "Canaux : " + Canaux + "\n";
            result += "Frequence : " + Frequence + "\n";
            result += "BitsPerSample : " + BitsPerSample + "\n";
            result += "BytePerBloc : " + BytePerBloc + "\n";
            result += "BytePerSec : " + BytePerSec + "\n";
            result += "Seconds : " + Seconds + "\n";
            return result;
        }
    }
}


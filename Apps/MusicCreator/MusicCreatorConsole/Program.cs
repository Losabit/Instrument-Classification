using MathNet.Numerics.Integration;
using NAudio.Wave;
using System;
using System.IO;
using AudioLibrary;
using MathNet.Numerics;

/// <summary>
/// Former un dataset automatiquement ++
/// 
/// Problèmes générations de sons
/// </summary>

namespace MusicCreatorConsole
{
    class Program
    {
        static void Main(string[] args)
        {
            try
            {
                #region Tentative de cération de sinus wave (sans reussite)
                //ffmpeg -f lavfi -i "sine=frequency=300:duration=5" 300.wav
                //int frequence = 80;
                //double[] fundamental = Generate.Sinusoidal(2000, 2000, frequence, 255);
                //WavFile wav = new WavFile(frequence);
                //wav.Canaux = 1;
                //byte[] data = new byte[fundamental.Length];
                //for (int i = 0; i < data.Length; i++)
                //{
                //    data[i] = (byte)fundamental[i];
                //}
                //wav.Data = data;
                //wav.Create(@"C:\Users\quent\Desktop\Projet Annuel\Applis\MusicCreator\MusicCreatorConsole\ressources\80.wav");
                #endregion

                #region Cut Sound
                //string musicDirectory = @"C:\Users\quent\Desktop\Projet Annuel\Musiques\Notes";
                //string directionDirectory = @"C:\Users\quent\Desktop\Projet Annuel\Applis\MusicCreator\MusicCreator\ressources\notes\guitare\strat";
                //string musicFile = @"strat.wav";
                //string musicPath = musicDirectory + "\\" + musicFile;
                //WavFile music = WavFile.Read(musicPath);
                //WavFile[] extraits = music.Cut(0, 2);

                //string[] pathsExtraits = Common.GenerateFileName(musicFile, extraits.Length);
                //for (int i = 0; i < extraits.Length; i++)
                //{
                //    extraits[i].Create(directionDirectory + "\\" + pathsExtraits[i]);
                //}
                #endregion

                #region Mix Generation

                //WavFile wav = WavFile.Read(@"C:\Users\quent\Desktop\Projet Annuel\Applis\MusicCreator\MusicCreatorConsole\ressources\test.wav");
                //wav.WhiteWave(4).Create(@"C:\Users\quent\Desktop\Projet Annuel\Applis\MusicCreator\MusicCreatorConsole\ressources\white.wav");
                //GeneratorInfo generatorInfo = new GeneratorInfo(Instrument.Guitare);
                //generatorInfo.InDirectory.Add(@"C:\Users\quent\Desktop\Projet Annuel\Applis\MusicCreator\MusicCreator\ressources\notes\guitare\nylon");
                //generatorInfo.InDirectory.Add(@"C:\Users\quent\Desktop\Projet Annuel\Applis\MusicCreator\MusicCreator\ressources\notes\basse");
                //generatorInfo.InDirectory.Add(@"C:\Users\quent\Desktop\Projet Annuel\Applis\MusicCreator\MusicCreator\ressources\notes\guitare\strat");
                //generatorInfo.OutDirectory = @"C:\Users\quent\Desktop\Projet Annuel\Applis\MusicCreator\MusicCreator\ressources\dataset";
                //generatorInfo.OutFileName = "mixed" + Enum.GetName(typeof(Instrument), Instrument.Guitare) + ".wav";
                //generatorInfo.NumberOfRecord = 45;
                //generatorInfo.MinMix = 3;
                //generatorInfo.MaxMix = 5;
                //DatasetGenerator.GenerateMix(generatorInfo);
                //WavFile[] waves = WavFile.ReadDirectory(generatorInfo.OutDirectory);
                //WavFile.Regroupe(waves).Create(generatorInfo.OutDirectory + "\\" + "long.wav");
                #endregion

                #region Stereo To Mono
                //string musicDirectory = @"C:\Users\quent\Desktop\Projet Annuel\Applis\MusicCreator\MusicCreator\ressources\musique";
                //string musicMonoDirectory = musicDirectory + @"\mono";
                //string musicFile = @"red-hot-chili-peppers-coffee-shop-official-music-video.wav";
                //string musicPath = musicDirectory + "\\" + musicFile;
                //string musicMonoPath = musicMonoDirectory + "\\" + musicFile;


                //WavFile wav = WavFile.Read(musicPath);
                //WavFile[] wavMono = wav.ToMono();

                //string[] pathsExtraits = Common.GenerateFileName(musicMonoPath, wavMono.Length);
                //for (int i = 0; i < wavMono.Length; i++)
                //    wavMono[i].Create(pathsExtraits[i]);
                #endregion

                Console.WriteLine("Programme terminé");
            }
            catch (Exception e)
            {
                Console.WriteLine(e.Message);
            }
            finally
            {
                Console.ReadKey();
            }
        }
    }
}

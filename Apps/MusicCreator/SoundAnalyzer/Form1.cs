using SoundAnalyzer.Models;
using MathNet.Numerics.IntegralTransforms;
using System;
using System.Numerics;
using System.Windows.Forms;
using AudioLibrary;
using System.Drawing;
using System.Windows.Forms.DataVisualization.Charting;
using System.IO;
using System.Collections.Generic;
using SoundAnalyzer.Forms;

namespace SoundAnalyzer
{
    public partial class Form1 : Form
    {
        public Form1()
        {
            InitializeComponent();
        }

        #region Analyse
        private void PlotWaveFile(WavFile wav, double start, double duree)
        {
            try
            {
                WavFile[] channels = wav.ToMono();
                chart1.Series.Clear();
                chart1.ChartAreas.Clear();
                checkedListBoxFourierCanaux.Items.Clear();

                for (int i = 0; i < channels.Length; i++)
                {
                    Series serie = new Series("Channel " + (i + 1));
                    serie.ChartType = SeriesChartType.FastLine;
                    ChartArea chartArea = new ChartArea("Chart" + serie.Name);
                    chart1.ChartAreas.Add(chartArea);
                    serie.ChartArea = "Chart" + serie.Name;
                    chart1.Series.Add(serie);
                    checkedListBoxFourierCanaux.Items.Add(serie.Name, i == 0 ? true : false);

                    int startIndice = channels[i].GetIndice(start) / (channels[i].BitsPerSample / 8);
                    int length = (channels[i].GetIndice(duree) / (channels[i].BitsPerSample / 8));
                    Complex[] complexs = Common.CutArray(channels[i].GetDecibelData(), startIndice, length);
                    for (int j = 1; j < complexs.Length; j++)
                        chart1.Series[i].Points.AddXY(Math.Round(channels[i].GetSecond((j + startIndice) * (channels[i].BitsPerSample / 8)), 5), complexs[j].Real);
                }
            }
            catch (Exception e)
            {
                MessageBox.Show(e.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        private void PlotFourierTransform(WavFile wav, string name, double start, double duree)
        {
            try
            {
                Series serie = new Series(name);
                serie.ChartType = SeriesChartType.FastLine;
                //chart1.ChartAreas[2].AxisX.MinorTickMark.Enabled = true;
                ChartArea chartArea = new ChartArea("Chart" + serie.Name);
                chart1.ChartAreas.Add(chartArea);
                serie.ChartArea = "Chart" + serie.Name;
                chart1.Series.Add(serie);

                int startIndice = wav.GetIndice(start) / (wav.BitsPerSample / 8);
                int length = (wav.GetIndice(duree) / (wav.BitsPerSample / 8));
                Complex[] complexs = Common.CutArray(wav.GetDecibelData(), startIndice, length);

                Fourier.Forward(complexs, FourierOptions.Default);
                var scale = Fourier.FrequencyScale(complexs.Length, wav.Frequence);
                for (int i = 0; i < complexs.Length; i++)
                {
                    if (scale[i] > 0)
                        chart1.Series[chart1.Series.Count - 1].Points.AddXY(scale[i], complexs[i].Magnitude);
                }
            }
            catch (Exception e)
            {
                MessageBox.Show(e.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        private void PlotInverseFourier(WavFile wav)
        {
            //Fourier.Inverse(complexs);
            //for (int i = 0; i < complexs.Length; i++)
            //    chart1.Series[1].Points.AddXY(Math.Round(channel1.GetSecond((i + start) * (channel1.BitsPerSample / 8)), 5), complexs[i].Real);
        }

        private int GetPic(Complex[] complexs)
        {
            double max = complexs[0].Magnitude;
            int indice = 0;
            for (int i = 1; i < complexs.Length; i++)
            {
                if (complexs[i].Magnitude > max)
                {
                    max = complexs[i].Magnitude;
                    indice = i;
                }
            }
            return indice;
        }

        private void AnalyseTab(WavFile wav)
        {
            try
            {
                double start = 0;
                double duree = 4;
                if (!double.TryParse(textBoxPlotStart.Text.Replace(".", ","), out start) || !double.TryParse(textBoxPlotDuree.Text.Replace(".", ","), out duree))
                    throw new Exception("Bad value for start or duree");

                buttonPlot.Click += new EventHandler(delegate (object senderChild, EventArgs eventArgs)
                {
                    try
                    {
                        if (!double.TryParse(textBoxPlotStart.Text.Replace(".", ","), out start) || !double.TryParse(textBoxPlotDuree.Text.Replace(".", ","), out duree))
                            throw new Exception("Bad value for start or duree");
                        PlotWaveFile(wav, start, duree);
                    }
                    catch (Exception e)
                    {
                        MessageBox.Show(e.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                    }
                });
                buttonFourrier.Click += new EventHandler(delegate (object senderChild, EventArgs eventArgs)
                {
                    try
                    {
                        double startFourier = 0;
                        double dureeFourier = 4;
                        if (!double.TryParse(textBoxFourierStart.Text.Replace(".", ","), out startFourier) || !double.TryParse(textBoxFourierDuree.Text.Replace(".", ","), out dureeFourier))
                            throw new Exception("Bad value for start or duree of Fourier");

                        WavFile[] channels = wav.ToMono();
                        for (int i = channels.Length; i < chart1.Series.Count; i++)
                        {
                            chart1.Series.RemoveAt(i);
                            chart1.ChartAreas.RemoveAt(i);
                            i--;
                        }
                        for (int i = 0; i < checkedListBoxFourierCanaux.CheckedIndices.Count; i++)
                        {
                            PlotFourierTransform(channels[checkedListBoxFourierCanaux.CheckedIndices[i]], "FFT" + (checkedListBoxFourierCanaux.CheckedIndices[i] + 1), startFourier, dureeFourier);
                        }
                    }
                    catch (Exception e)
                    {
                        MessageBox.Show(e.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                    }
                });
                PlotWaveFile(wav, start, duree);
            }
            catch (Exception e)
            {
                MessageBox.Show(e.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }
        #endregion

        private void listFile_Click(object sender, MouseEventArgs e)
        {
            if (e.Button == MouseButtons.Right)
            {
                ContextMenu m = new ContextMenu();
                MenuItem importItem = new MenuItem("Import");
                importItem.Click += new EventHandler(delegate (object senderChild, EventArgs eventArgs)
                {
                    var openFileDialog = new OpenFileDialog();
                    openFileDialog.Multiselect = true;
                    var result = openFileDialog.ShowDialog();
                    if (result == DialogResult.OK)
                        Import(openFileDialog.FileNames);
                });
                m.MenuItems.Add(importItem);

                MenuItem importFolderItem = new MenuItem("ImportFolder");
                importFolderItem.Click += new EventHandler(delegate (object senderChild, EventArgs eventArgs)
                {
                    var folderBrowserDialog = new FolderBrowserDialog();
                    var result = folderBrowserDialog.ShowDialog();
                    if (result != DialogResult.OK)
                        return;
                    DirectoryInfo d = new DirectoryInfo(folderBrowserDialog.SelectedPath);
                    List<string> files = new List<string>();
                    List<DirectoryInfo> directories = Common.GetDirectoryRecursively(d);

                    for (int i = 0; i < directories.Count; i++)
                    {
                        FileInfo[] filesMp3 = directories[i].GetFiles("*.mp3");
                        FileInfo[] filesWav = directories[i].GetFiles("*.wav");
                        foreach (FileInfo fileMp3 in filesMp3)
                        {
                            files.Add(fileMp3.FullName);
                        }
                        foreach (FileInfo fileWav in filesWav)
                        {
                            files.Add(fileWav.FullName);
                        }
                    }
                    Import(files.ToArray());
                });
                m.MenuItems.Add(importFolderItem);

                MenuItem cleanItem = new MenuItem("Clean");

                if (listFile.SelectedItems.Count >= 1)
                {
                    cleanItem.Click += new EventHandler(delegate (object senderChild, EventArgs eventargs)
                    {
                        for (int i = 0; i < listFile.SelectedItems.Count; i++)
                        {
                            listFile.Items.Remove(listFile.SelectedItems[i]);
                            i--;
                        }
                    });

                    MenuItem splitItem = new MenuItem("Split");
                    splitItem.Click += new EventHandler(delegate (object senderChild, EventArgs eventargs)
                    {
                        var formPopup = new SplitForm(listFile);
                        formPopup.Show(this);
                    });
                    m.MenuItems.Add(splitItem);

                    if (listFile.SelectedItems.Count == 1)
                    {
                        MenuItem analyseItem = new MenuItem("Analyse");
                        analyseItem.Click += new EventHandler(delegate (object senderChild, EventArgs eventargs)
                        {
                            tabControl1.SelectTab(tabPage2);
                            WavFile wav = WavFile.Read(((FileImport)listFile.SelectedItem).Path);
                            AnalyseTab(wav);
                        });
                        m.MenuItems.Add(analyseItem);
                    }
                }
                else
                {
                    cleanItem.Click += new EventHandler(delegate (object senderChild, EventArgs eventargs)
                    {
                        listFile.Items.Clear();
                    });
                }

                m.MenuItems.Add(cleanItem);
                m.Show(listFile, new Point(e.X, e.Y));
            }
        }

        private void listFile_SelectedItemChanged(object sender, EventArgs e)
        {
            if (tabControl2.SelectedTab == tabPage6)
            {
                tabPage6_Load(sender, e);
            }
        }

        private void Import(string[] paths)
        {
            try
            {
                for (int i = 0; i < paths.Length; i++)
                {
                    string file = paths[i].Substring(paths[i].LastIndexOf(Path.DirectorySeparatorChar) + 1);
                    if (!file.Contains(".wav"))
                    {
                        if (file.Contains(".mp3"))
                        {
                            Common.ConvertMp3ToWav(paths[i], paths[i].Replace(".mp3", ".wav"));
                        }
                        else
                        {
                            throw new NotImplementedException("Format non geré par l'application veuillez utiliser un format mp3 ou wav (ou demander à Quentin de l'ajouter ;))");
                        }
                    }
                    listFile.Items.Add(new FileImport
                    {
                        Path = paths[i].Replace(".mp3", ".wav"),
                        File = file.Replace(".mp3", ".wav")
                    });
                }
                listFile.DisplayMember = "File";
                listFile.ValueMember = "Path";
            }
            catch (Exception e)
            {
                MessageBox.Show(e.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        #region Info Tab

        private void tabPage6_Load(object sender, EventArgs e)
        {
            if (listFile.SelectedItems.Count == 1)
            {
                WavFile wav = WavFile.Read(((FileImport)listFile.SelectedItem).Path);
                labelName.Text = ((FileImport)listFile.SelectedItem).File;
                labelValueCanaux.Text = wav.Canaux.ToString();
                labelValueFrequence.Text = wav.Frequence.ToString() + " Hz";
                labelValueBit.Text = wav.BitsPerSample.ToString();
                labelValueDuree.Text = wav.Seconds.ToString("0.00") + " seconds";
            }
            else
            {
                labelName.Text = string.Empty;
                labelValueCanaux.Text = string.Empty;
                labelValueFrequence.Text = string.Empty;
                labelValueBit.Text = string.Empty;
                labelValueDuree.Text = string.Empty;
            }
        }

        private void button1_Click_1(object sender, EventArgs e)
        {
            if (listFile.SelectedItems.Count == 1)
            {
                tabControl1.SelectTab(tabPage2);
                WavFile wav = WavFile.Read(((FileImport)listFile.SelectedItem).Path);
                AnalyseTab(wav);
            }
        }

        #endregion

        #region Cut Tab

        private void buttonPath_Click(object sender, EventArgs e)
        {
            var openFileDialog = new FolderBrowserDialog();
            var result = openFileDialog.ShowDialog();
            if (result == DialogResult.OK)
                textBoxPath.Text = openFileDialog.SelectedPath;
        }

        private void buttonCut_Click(object sender, EventArgs e)
        {
            try
            {
                double start;
                double end;
                int morceaux;
                if (listFile.SelectedItems.Count == 0)
                    throw new Exception("No items selected");

                if (!double.TryParse(textBoxCutStart.Text.Replace(".", ","), out start) || !double.TryParse(textBoxCutEnd.Text.Replace(".", ","), out end) || !int.TryParse(textBoxCutMorceaux.Text, out morceaux))
                    throw new Exception("Start need double \nEnd need double \nMorceaux need int");

                for (int i = 0; i < listFile.SelectedItems.Count; i++)
                {
                    FileImport file = (FileImport)listFile.SelectedItems[i];
                    WavFile wav = WavFile.Read(file.Path);
                    WavFile[] extraits = wav.Cut(start, end, morceaux);
                    for (int j = 0; j < extraits.Length; j++)
                    {
                        double startExtrait = start + ((end - start) / morceaux) * j;
                        double endExtrait = startExtrait + ((end - start) / morceaux);
                        string path = textBoxPath.Text + Path.DirectorySeparatorChar + file.File.Replace(".wav", "Cut" + startExtrait.ToString("0.0") + "_" + endExtrait.ToString("0.0") + ".wav");
                        extraits[j].Create(path);

                        if (checkBoxCutImport.Checked)
                        {
                            listFile.Items.Add(new FileImport
                            {
                                Path = path,
                                File = file.File + "Cut" + startExtrait + "_" + endExtrait + ".wav"
                            });
                        }
                    }
                }

            }
            catch (Exception exception)
            {
                MessageBox.Show(exception.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        #endregion

        #region Dataset Tab
        private void tabPage3_Load(object sender, EventArgs e)
        {
            comboBoxDatasetEtiquette.DataSource = Enum.GetValues(typeof(DatasetGenerator.Instrument));
        }

        private void buttonClean_Click(object sender, EventArgs e)
        {
            textBoxCutStart.Text = string.Empty;
            textBoxCutEnd.Text = string.Empty;
            textBoxCutMorceaux.Text = string.Empty;
            checkBoxCutImport.Checked = false;
        }

        private void buttonDataset_Click(object sender, EventArgs e)
        {
            DatasetGenerator.Instrument instrument;
            if (!Enum.TryParse(comboBoxDatasetEtiquette.SelectedValue.ToString(), out instrument) || listFile.SelectedItems.Count == 0)
                return;

            List<string> paths = new List<string>();
            for (int i = 0; i < listFile.SelectedItems.Count; i++)
            {
                paths.Add(((FileImport)listFile.SelectedItems[i]).Path);
            }

            DatasetGenerator dataset = new DatasetGenerator(instrument, paths);
        }
        #endregion
    }
}

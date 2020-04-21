using AudioLibrary;
using SoundAnalyzer.Models;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace SoundAnalyzer.Forms
{
    public partial class SplitForm : Form
    {
        private ListBox listBox;

        public SplitForm(ListBox listBox)
        {
            InitializeComponent();
            this.listBox = listBox;
        }

        private void buttonSplit_Click(object sender, EventArgs e)
        {
            int nbCanaux = -1;
            if (!int.TryParse(textBox1.Text, out nbCanaux))
                return;

            var folderBrowserDialog = new FolderBrowserDialog();
            var result = folderBrowserDialog.ShowDialog();
            if (result != DialogResult.OK)
                return;

            for (int i = 0; i < listBox.SelectedItems.Count; i++)
            {
                WavFile wav = WavFile.Read(((FileImport)listBox.SelectedItems[i]).Path);
                if (wav.Canaux == 1)
                    continue;

                WavFile[] channels = wav.ToMono();
                for (int j = 0; j < nbCanaux; j++)
                {
                    string path = folderBrowserDialog.SelectedPath + Path.DirectorySeparatorChar + ((FileImport)listBox.SelectedItems[i]).File;
                    if (File.Exists(path.Replace(".wav", "_" + j + ".wav")))
                    {
                        Random rand = new Random();
                        channels[j].Create(path.Replace(".wav", "_" + (j + rand.Next(100)).ToString() + ".wav"));
                    }
                    else
                        channels[j].Create(path.Replace(".wav", "_" + j + ".wav"));

                    if (j < nbCanaux)
                    {
                        listBox.Items.Add(new FileImport
                        {
                            Path = path.Replace(".wav", "_" + j + ".wav"),
                            File = path.Replace(".wav", "_" + j + ".wav").Substring(path.LastIndexOf(Path.DirectorySeparatorChar) + 1)
                        });
                    }
                }
            }
            Close();
        }
    }
}

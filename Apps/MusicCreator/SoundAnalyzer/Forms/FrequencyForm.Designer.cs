namespace SoundAnalyzer.Forms
{
    partial class FrequencyForm
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.label1 = new System.Windows.Forms.Label();
            this.buttonFrequency = new System.Windows.Forms.Button();
            this.buttonPath = new System.Windows.Forms.Button();
            this.textBoxFrequence = new System.Windows.Forms.TextBox();
            this.label2 = new System.Windows.Forms.Label();
            this.textBoxPath = new System.Windows.Forms.TextBox();
            this.SuspendLayout();
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.Font = new System.Drawing.Font("Microsoft Sans Serif", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label1.Location = new System.Drawing.Point(107, 100);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(167, 20);
            this.label1.TabIndex = 0;
            this.label1.Text = "Nouvelle Fréquence :";
            // 
            // buttonFrequency
            // 
            this.buttonFrequency.Location = new System.Drawing.Point(280, 268);
            this.buttonFrequency.Name = "buttonFrequency";
            this.buttonFrequency.Size = new System.Drawing.Size(161, 62);
            this.buttonFrequency.TabIndex = 2;
            this.buttonFrequency.Text = "Créer";
            this.buttonFrequency.UseVisualStyleBackColor = true;
            this.buttonFrequency.Click += new System.EventHandler(this.buttonFrequency_Click);
            // 
            // buttonPath
            // 
            this.buttonPath.Location = new System.Drawing.Point(538, 188);
            this.buttonPath.Name = "buttonPath";
            this.buttonPath.Size = new System.Drawing.Size(42, 23);
            this.buttonPath.TabIndex = 3;
            this.buttonPath.Text = "...";
            this.buttonPath.UseVisualStyleBackColor = true;
            this.buttonPath.Click += new System.EventHandler(this.buttonPath_Click);
            // 
            // textBoxFrequence
            // 
            this.textBoxFrequence.Location = new System.Drawing.Point(280, 100);
            this.textBoxFrequence.Name = "textBoxFrequence";
            this.textBoxFrequence.Size = new System.Drawing.Size(240, 22);
            this.textBoxFrequence.TabIndex = 4;
            // 
            // label2
            // 
            this.label2.AutoSize = true;
            this.label2.Font = new System.Drawing.Font("Microsoft Sans Serif", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label2.Location = new System.Drawing.Point(107, 191);
            this.label2.Name = "label2";
            this.label2.Size = new System.Drawing.Size(167, 20);
            this.label2.TabIndex = 5;
            this.label2.Text = "Répertoire de sortie :";
            // 
            // textBoxPath
            // 
            this.textBoxPath.Location = new System.Drawing.Point(280, 189);
            this.textBoxPath.Name = "textBoxPath";
            this.textBoxPath.Size = new System.Drawing.Size(240, 22);
            this.textBoxPath.TabIndex = 6;
            // 
            // FrequencyForm
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 16F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(800, 450);
            this.Controls.Add(this.textBoxPath);
            this.Controls.Add(this.label2);
            this.Controls.Add(this.textBoxFrequence);
            this.Controls.Add(this.buttonPath);
            this.Controls.Add(this.buttonFrequency);
            this.Controls.Add(this.label1);
            this.Name = "FrequencyForm";
            this.Text = "FrequencyForm";
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.Button buttonFrequency;
        private System.Windows.Forms.Button buttonPath;
        private System.Windows.Forms.TextBox textBoxFrequence;
        private System.Windows.Forms.Label label2;
        private System.Windows.Forms.TextBox textBoxPath;
    }
}
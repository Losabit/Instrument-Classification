	¦Σ�qR@¦Σ�qR@!¦Σ�qR@	�02�{n�?�02�{n�?!�02�{n�?"e
=type.googleapis.com/tensorflow.profiler.PerGenericStepDetails$¦Σ�qR@IIC���?A�t�i�mR@YQlMK��?*	_��"�IE@2j
3Iterator::Model::ParallelMap::Zip[1]::ForeverRepeatI�V��?!($���=@)(,�)�?1u�J�:@:Preprocessing2F
Iterator::Modelz4Փ�G�?!�i����A@)ѕT� �?1 ���b�4@:Preprocessing2t
=Iterator::Model::ParallelMap::Zip[0]::FlatMap[0]::Concatenate�ڧ�1�?!�C#��8@)W|C�u�?1���[�2@:Preprocessing2S
Iterator::Model::ParallelMapS=��Mz?!^�,�*.@)S=��Mz?1^�,�*.@:Preprocessing2X
!Iterator::Model::ParallelMap::Zip*�TPQ��?!4�<1P@)����c?1��4�'�@:Preprocessing2�
MIterator::Model::ParallelMap::Zip[0]::FlatMap[0]::Concatenate[0]::TensorSlice:y�	�5b?!��o�@):y�	�5b?1��o�@:Preprocessing2d
-Iterator::Model::ParallelMap::Zip[0]::FlatMapH�C��݈?!��R䊄<@)]���^?1n���O�@:Preprocessing2v
?Iterator::Model::ParallelMap::Zip[1]::ForeverRepeat::FromTensor��p���W?!����j+@)��p���W?1����j+@:Preprocessing:�
]Enqueuing data: you may want to combine small input data chunks into fewer but larger chunks.
�Data preprocessing: you may increase num_parallel_calls in <a href="https://www.tensorflow.org/api_docs/python/tf/data/Dataset#map" target="_blank">Dataset map()</a> or preprocess the data OFFLINE.
�Reading data from files in advance: you may tune parameters in the following tf.data API (<a href="https://www.tensorflow.org/api_docs/python/tf/data/Dataset#prefetch" target="_blank">prefetch size</a>, <a href="https://www.tensorflow.org/api_docs/python/tf/data/Dataset#interleave" target="_blank">interleave cycle_length</a>, <a href="https://www.tensorflow.org/api_docs/python/tf/data/TFRecordDataset#class_tfrecorddataset" target="_blank">reader buffer_size</a>)
�Reading data from files on demand: you should read data IN ADVANCE using the following tf.data API (<a href="https://www.tensorflow.org/api_docs/python/tf/data/Dataset#prefetch" target="_blank">prefetch</a>, <a href="https://www.tensorflow.org/api_docs/python/tf/data/Dataset#interleave" target="_blank">interleave</a>, <a href="https://www.tensorflow.org/api_docs/python/tf/data/TFRecordDataset#class_tfrecorddataset" target="_blank">reader buffer</a>)
�Other data reading or processing: you may consider using the <a href="https://www.tensorflow.org/programmers_guide/datasets" target="_blank">tf.data API</a> (if you are not using it now)�
:type.googleapis.com/tensorflow.profiler.BottleneckAnalysis�
device�Your program is NOT input-bound because only 0.0% of the total step time sampled is waiting for input. Therefore, you should focus on reducing other time.no*no#You may skip the rest of this page.B�
@type.googleapis.com/tensorflow.profiler.GenericStepTimeBreakdown�
	IIC���?IIC���?!IIC���?      ��!       "      ��!       *      ��!       2	�t�i�mR@�t�i�mR@!�t�i�mR@:      ��!       B      ��!       J	QlMK��?QlMK��?!QlMK��?R      ��!       Z	QlMK��?QlMK��?!QlMK��?JCPU_ONLY
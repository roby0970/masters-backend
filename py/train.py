from os import name, read
from sklearn.neighbors import KNeighborsClassifier
import sys
import pandas as pd
import json
from string import Template

#file = 'C:/diplomski/blefingerprinting android/backed/backend/datasets/home_dataset.csv'




def classify(indexes, values):
    
    x = dataset.iloc[:, indexes].values
    x = x.astype(int)
        
    y = dataset.iloc[:, column_numb-1 ].values
    
    neigh = KNeighborsClassifier(n_neighbors=1)
    neigh.fit(x, y)
    
    classification = neigh.predict([values])
    
    coordX , coordY = classification[0].split('-')
    
    result = json.dumps({"x": int(coordX), "y" : int(coordY)}) 
    print(result)
    

if len(sys.argv) == 3:
    #read dataset and its columns
    file = open(sys.argv[2], 'r')
    first_line = file.readline()
    column_numb = len(first_line.split(','))
    
    columns = ['ble'+ str(i+1)  for i in range(column_numb-1)] + ['coords']
    dataset = pd.read_csv(sys.argv[2], names=columns, encoding='UTF-16 LE')
    readings = json.loads(sys.argv[1])
    indexes = list()
    values = list()
    for e in readings:
        indexes.append(e["id_ble"] - 1)
        values.append(e["rssi"])
    
    classify(indexes, values)

